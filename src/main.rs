use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use grpc_proto::pb::{CacheKvRequest, CacheKvResponse, HelloRequest, HelloResponse};
use grpc_proto::pb::{FILE_DESCRIPTOR_SET, hello_service_server::HelloService, hello_service_server::HelloServiceServer};
use lazy_static::initialize;
use opentelemetry::{Context, global, KeyValue, trace};
use opentelemetry::trace::{TraceContextExt, Tracer};
use redis::{Commands, Connection, ConnectionAddr};
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status, transport::Server};
use tonic_reflection::server;
use tracing::{info, Span};
use tracing::instrument::WithSubscriber;
use tracing_attributes::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::fmt::SubscriberBuilder;

use crate::config::CONFIG;

mod mytracer;
mod config;
mod interceptor;

pub struct MyGreeter {
    redis_con: Arc<Mutex<Connection>>,
}

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);


#[tonic::async_trait]
impl HelloService for MyGreeter {
    async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        // request.extensions()
        let mut response_str = String::from("Hello");
        response_str.push_str(request.into_inner().name.as_str());

        Ok(Response::new(HelloResponse { message: response_str.parse().unwrap() }))
    }

    #[instrument( skip(self, request), fields(trace_id))]
    async fn cache_kv(&self, mut request: Request<CacheKvRequest>) -> Result<Response<CacheKvResponse>, Status> {
        let mut redis_con = self.redis_con.try_lock().unwrap();
        let parent_cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&interceptor::MyExtractor(&request))
        });
        let traceId = parent_cx.span().span_context().trace_id();
        Span::current().set_parent(parent_cx);
        Span::current().record("trace_id", &traceId.to_string().as_str());

       // ctx.metadata. (eyValue::new("trace_id", parent_cx.span().span_context().trace_id().to_string()));

        let request_msg = request.into_inner();

        info!("go to set redis kv {:?},{:?}",request_msg,traceId);

        match redis_con.set_ex::<_, _, ()>(request_msg.key, request_msg.value, request_msg.timeout as usize) {
            Ok(_) => {
                Ok(Response::new(CacheKvResponse { message: "set cache success".parse().unwrap() }))
            }
            Err(err) => {
                Err(Status::new(Code::Unavailable, err.to_string()))
            }
        }
    }
}

impl MyGreeter {
    fn new(redis_con: Arc<Mutex<Connection>>) -> MyGreeter {
        MyGreeter {
            redis_con,
        }
    }
}

fn init_redis(config : config::RedisConfig) -> Arc<Mutex<Connection>> {
    info!("going to init redis{:?}",config);
    //"redis://default:redispw@localhost:55000"
    let redis_con = redis::Client::open(redis::ConnectionInfo{
        addr: ConnectionAddr::Tcp(config.host, config.port),
        redis: redis::RedisConnectionInfo{
            db: 0,
            username: Option::from(config.user),
            password: Option::from(config.password),
        },
    }).expect("panic, redis connect failed").get_connection().expect("failed to connect to redis");
    Arc::new(Mutex::new(redis_con))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize(&config::CONFIG);
    //init_logger();
    mytracer::init_global_tracer();

    let addr= SocketAddr::from(([0, 0, 0, 0], CONFIG.port as u16));
    let redis_con = init_redis(config::CONFIG.redis_config.clone());




    let reflection = server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // The stack of middleware that our service will be wrapped in
    let layer = tower::ServiceBuilder::new()
         // Apply middleware from tower
        .timeout(Duration::from_secs(30))
        .layer(tonic::service::interceptor(interceptor::MyInterceptor::default()))
        .into_inner();

    Server::builder()
        .layer(tonic::service::interceptor(interceptor::MyInterceptor::default()))
        .add_service(reflection)
        .add_service(HelloServiceServer::new(MyGreeter::new(redis_con)))
        .serve(addr)

        .await?;
    Ok(())
}




use std::net::SocketAddr;
use std::sync::Arc;

use grpc_proto::pb::{CacheKvRequest, CacheKvResponse, HelloRequest, HelloResponse};
use grpc_proto::pb::{FILE_DESCRIPTOR_SET, hello_service_server::HelloService, hello_service_server::HelloServiceServer};
use lazy_static::initialize;
use redis::{Commands, Connection, ConnectionAddr, RedisConnectionInfo};
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status, transport::Server};
use tonic_reflection::server;
use tracing_attributes::instrument;
use crate::config::CONFIG;

mod mytracer;
mod config;

pub struct MyGreeter {
    redis_con: Arc<Mutex<Connection>>,
}

#[tonic::async_trait]
impl HelloService for MyGreeter {
    async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        // request.extensions()
        let mut response_str = String::from("Hello");
        response_str.push_str(request.into_inner().name.as_str());

        Ok(Response::new(HelloResponse { message: response_str.parse().unwrap() }))
    }

    #[instrument(skip(self))]
    async fn cache_kv(&self, request: Request<CacheKvRequest>) -> Result<Response<CacheKvResponse>, Status> {
        let mut redis_con = self.redis_con.try_lock().unwrap();
        let request_msg = request.into_inner();
        log::info!("go to set redis kv {:?}",request_msg);

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
    println!("{:?}",config);
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
    mytracer::init_global_tracer();
    initialize(&config::CONFIG);
    //env_logger::builder().filter_level(LevelFilter::Info).init();
    //let addr = "0.0.0.0:50051".parse()?;
    let addr= SocketAddr::from(([0, 0, 0, 0], CONFIG.port as u16));
    let redis_con = init_redis(config::CONFIG.redis_config.clone());


    let greeter = MyGreeter::new(redis_con);

    let reflection = server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection)
        .add_service(HelloServiceServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}


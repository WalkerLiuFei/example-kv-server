use std::sync::Arc;

use grpc_proto::pb::{CacheKvRequest, CacheKvResponse, hello_service_server, HelloRequest, HelloResponse};
use grpc_proto::pb::hello_service_server::HelloService;
use grpc_proto::pb::{hello_service_server::HelloServiceServer,FILE_DESCRIPTOR_SET};
use grpc_proto::pb;

use redis::{Commands, Connection};
use tokio::sync::Mutex;
use tonic;
use tonic::{Code, Request, Response, Status, transport::Server};
use tonic::codegen::http::request;
use tonic_reflection::server;

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

    async fn cache_kv(&self, request: Request<CacheKvRequest>) -> Result<Response<CacheKvResponse>, Status> {
        let mut redis_con = self.redis_con.try_lock().unwrap();
        let request_msg = request.into_inner();
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
        return MyGreeter {
            redis_con,
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "0.0.0.0:50051".parse()?;

    let redis_con = Arc::new(Mutex::new(redis::Client::open("redis://default:redispw@localhost:55000")?.get_connection()?));
    let mut greeter = MyGreeter::new(redis_con);

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
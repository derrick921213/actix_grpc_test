use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

pub fn grpc_service() -> GreeterServer<MyGreeter> {
    GreeterServer::new(MyGreeter)
}

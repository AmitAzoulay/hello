use tonic::{transport::{Server, ServerTlsConfig}, Request, Response, Status};

mod hello;
use crate::hello::say_server::{Say, SayServer};
use crate::hello::{SayRequest, SayResponse};

#[derive(Default)]
pub struct MySay;

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, req: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse { message: format!("hello {}", req.get_ref().name) }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    println!("Server listening on {addr}");
    Server::builder()
        .add_service(SayServer::new(MySay::default()))
        .serve(addr)
        .await?;
    Ok(())
}

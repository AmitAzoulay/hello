use tonic::{transport::{Server, Identity, ServerTlsConfig, Certificate}, Request, Response, Status};

mod hello;
use crate::hello::say_server::{Say, SayServer};
use crate::hello::{SayRequest, SayResponse};
use std::fs;

#[derive(Default)]
pub struct MySay;

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, req: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("Got a TLS-secured request from {:?}", req.remote_addr());

        let reply = SayResponse {
            message: format!("Hello {} from a TLS-enabled server!", req.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\server.pem")?;
    let server_key = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\server.key")?;
    let ca_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\ca.pem")?;

    let tls = ServerTlsConfig::new()
        .identity(Identity::from_pem(server_pem, server_key))
        .client_ca_root(Certificate::from_pem(ca_pem));


    let addr = "[::1]:50051".parse()?;
    println!("Server listening on {addr}");
    Server::builder()
        .tls_config(tls)?
        .add_service(SayServer::new(MySay::default()))
        .serve(addr)
        .await?;
    Ok(())
}

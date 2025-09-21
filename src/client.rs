use tonic::{transport::{Channel, Certificate, ClientTlsConfig, Identity}, Request, Status};

use hello::say_client::SayClient;
use hello::SayRequest;
use std::fs;


mod hello;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load CA certificate
    let ca_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\ca.pem")?;
    let client_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\client.pem")?;
    let client_key = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\client.key")?;

    // Create client TLS configuration
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(ca_pem))
        .identity(Identity::from_pem(client_pem, client_key))
        .domain_name("localhost");


    let channel = Channel::from_static("https://[::1]:50051")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = SayClient::new(channel);

    let request = tonic::Request::new(
        SayRequest {
            name:String::from("amit")
        },
    );

    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
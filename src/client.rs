use tonic::{transport::{Channel, Certificate, ClientTlsConfig, Identity}, Request, Status};
use std::sync::Arc;

use hello::say_client::SayClient;
use hello::SayRequest;


mod hello;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load CA certificate
    let ca_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\ca.pem")?;
    let client_pem = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\client.pem")?;
    let client_key = std::fs::read("C:\\Users\\Shuly\\Desktop\\hello\\certs\\client.key")?;

    let key_log = Arc::new(rustls::KeyLogFile::new());

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
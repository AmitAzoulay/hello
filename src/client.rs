use tonic::{transport::{Channel, Certificate, ClientTlsConfig, Identity}, Request, Status};
use std::sync::Arc;
use std::io::{self, Write};

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

    loop {
        println!("\nOptions:");
        println!("1. Create shape: [circle, rectangle, triangle]");
        println!("2. Add attribute: [color, frame, rotate]");
        println!("3. Print shapes");
        println!("e. Exit");

        print!("Enter command: ");
        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        if command == "e"
        {
            println!("Exiting...");
            break;
        }

        let request = Request::new(SayRequest
        {
            name: command.to_string(),
        });

        let response = client.send(request).await?.into_inner();
        println!("Server Response: {}", response.message);
        if command == "print"
        {
            for (i, shape) in response.shapes.iter().enumerate()
            {
                println!("Shape {}:\n{}\n", i + 1, shape);
            }
        }
    }
    Ok(())
}
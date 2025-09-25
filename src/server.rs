use std::io::WriterPanicked;
use tonic::{transport::{Server, Identity, ServerTlsConfig, Certificate}, Request, Response, Status};

mod hello;
use crate::hello::say_server::{Say, SayServer};
use crate::hello::{SayRequest, SayResponse};

pub trait Shape
{
    fn calcArea(&self) -> f64;
    fn calcPerimeter(&self) -> f64;
    fn Tostring(&self) -> String;
}

struct Circle
{
    radius: f64,
}
impl Circle
{
    fn createShape(radius: f64) -> Circle
    {
        Circle{radius: radius}
    }
}
impl Shape for Circle
{
    fn calcArea(&self) -> f64
    {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn calcPerimeter(&self) -> f64
    {
        2.0 * std::f64::consts::PI * self.radius
    }
    fn Tostring(&self) -> String
    {
        format!("Circle radius: {}\nCircle area: {}\nCircle perimeter: {}", self.radius, self.calcArea(), self.calcPerimeter())
    }
}
struct Rectangle
{
    width: f64,
    height: f64,
}
impl Rectangle
{
    fn createShape(width: f64, height: f64) -> Rectangle
    {
        Rectangle{width: width, height: height}
    }
}
impl Shape for Rectangle{
    fn calcArea(&self) -> f64
    {
        self.width * self.height
    }
    fn calcPerimeter(&self) -> f64
    {
        self.width * self.height * 2.0
    }
    fn Tostring(&self) -> String
    {
        format!("Rectangle width: {}\nheight: {}\nRectangle area: {}\nRectangle perimeter: {}", self.width, self.height, self.calcArea(), self.calcPerimeter())
    }
}

struct Triangle
{
    b: f64,
    a: f64,
    c: f64
}
impl Triangle
{
    fn createShape(a: f64, b: f64, c: f64) -> Triangle
    {
        Triangle{b:b, a:a, c: c}
    }
}
impl Shape for Triangle
{
    fn calcArea(&self) -> f64
    {
       (2.0 * (self.a / self.b)) * self.b/ 2.0
    }
    fn calcPerimeter(&self) -> f64
    {
        self.a + self.c + self.b
    }
    fn Tostring(&self) -> String
    {
        format!("Triangle width: {}\nTriangle height: {}\nTriangle area: {}\nTriangle perimeter: {}", self.b, (2.0 * (self.a / self.b)), self.calcArea(), self.calcPerimeter())
    }
}















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

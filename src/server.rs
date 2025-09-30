use tonic::{transport::{Server, Identity, ServerTlsConfig, Certificate}, Request, Response, Status};
use std::sync::Mutex;
use once_cell::sync::Lazy;
mod hello;
use crate::hello::say_server::{Say, SayServer};
use crate::hello::{SayRequest, SayResponse};


static SHAPES: Lazy<Mutex<Vec<Box<dyn Shape + Send + Sync>>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});
pub trait Shape: Send + Sync
{
    fn calcArea(&self) -> f64;
    fn calcPerimeter(&self) -> f64;
    fn Tostring(&self) -> String;
    fn shape_type(&self) -> String;
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
    fn shape_type(&self) -> String
    {
        "circle".to_string()
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
        (self.width + self.height) * 2.0
    }
    fn Tostring(&self) -> String
    {
        format!("Rectangle width: {}\nheight: {}\nRectangle area: {}\nRectangle perimeter: {}", self.width, self.height, self.calcArea(), self.calcPerimeter())
    }
    fn shape_type(&self) -> String
    {
        "rectangle".to_string()
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
    fn shape_type(&self) -> String
    {
        "triangle".to_string()
    }
}

struct ColoredShape
{
    shape: Box<dyn Shape>,
    color: String,
}
impl ColoredShape
{
    fn createShape(shape: Box<dyn Shape>, color: String) -> ColoredShape
    {
        ColoredShape{shape: shape, color: color}
    }
}
impl Shape for ColoredShape
{
    fn calcArea(&self) -> f64
    {
        self.shape.calcArea()
    }
    fn calcPerimeter(&self) -> f64
    {
        self.shape.calcPerimeter()
    }
    fn Tostring(&self) -> String
    {
        format!("coloredShape:{}\ncolor: {}", self.shape.Tostring(), self.color)
    }
    fn shape_type(&self) -> String
    {
        self.shape.shape_type()
    }
}
struct FramedShape
{
    shape: Box<dyn Shape>,
    frame_thickness: f64,
}
impl FramedShape
{
    fn createShape(shape: Box<dyn Shape>, frame_thickness: f64) -> FramedShape
    {
        FramedShape { shape, frame_thickness }
    }
}
impl Shape for FramedShape {
    fn calcArea(&self) -> f64
    {
        self.shape.calcArea()
    }
    fn calcPerimeter(&self) -> f64
    {
        self.shape.calcPerimeter()
    }
    fn Tostring(&self) -> String
    {
        format!(
            "FramedShape:\n{}\nFrame thickness: {}",
            self.shape.Tostring(),
            self.frame_thickness,
        )
    }
    fn shape_type(&self) -> String
    {
        self.shape.shape_type()
    }
}
struct RotatedShape
{
    shape: Box<dyn Shape>,
    degrees: f64,
}

impl RotatedShape
{
    fn createShape(shape: Box<dyn Shape>, degrees: f64) -> RotatedShape
    {
        RotatedShape { shape, degrees }
    }
}

impl Shape for RotatedShape
{
    fn calcArea(&self) -> f64
    {
        self.shape.calcArea()
    }
    fn calcPerimeter(&self) -> f64
    {
        self.shape.calcPerimeter()
    }
    fn Tostring(&self) -> String
    {
        format!(
            "RotatedShape:\n{}\nRotation: {}",
            self.shape.Tostring(),
            self.degrees
        )
    }
    fn shape_type(&self) -> String
    {
        self.shape.shape_type()
    }
}

#[derive(Default)]
pub struct MySay;
#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, req: Request<SayRequest>) -> Result<Response<SayResponse>, Status>
    {
        println!("Got a TLS-secured request from {:?}", req.remote_addr());

        let command = req.into_inner().name;
        let mut shapes = SHAPES.lock().unwrap();

        let mut reply_msg = String::new();

        let tokens: Vec<&str> = command.split_whitespace().collect();
        match tokens.as_slice()
        {
            ["circle"] => {
                shapes.push(Box::new(Circle::createShape(5.0)));
                reply_msg = "Added a circle.".to_string();
            }
            ["rectangle"] => {
                shapes.push(Box::new(Rectangle::createShape(4.0, 3.0)));
                reply_msg = "Added a rectangle.".to_string();
            }
            ["triangle"] => {
                shapes.push(Box::new(Triangle::createShape(3.0, 4.0, 5.0)));
                reply_msg = "Added a triangle.".to_string();
            }
            ["color", color] => {
                if let Some(last) = shapes.pop()
                {
                    let colored = Box::new(ColoredShape::createShape(last, color.to_string()));
                    shapes.push(colored);
                    reply_msg = format!("Applied color: {}", color);
                }
                else
                {
                    reply_msg = "No shape to color.".to_string();
                }
            }
            ["frame", thickness] => {
                if let Ok(t) = thickness.parse::<f64>()
                {
                    if let Some(last) = shapes.pop()
                    {
                        let framed = Box::new(FramedShape::createShape(last, t));
                        shapes.push(framed);
                        reply_msg = format!("Applied frame thickness: {}", t);
                    }
                    else
                    {
                        reply_msg = "No shape to frame.".to_string();
                    }
                }
                else
                {
                    reply_msg = "Invalid thickness.".to_string();
                }
            }
            ["rotate", degrees] =>
                {
                if let Ok(d) = degrees.parse::<f64>()
                {
                    if let Some(last) = shapes.pop()
                    {
                        let rotated = Box::new(RotatedShape::createShape(last, d));
                        shapes.push(rotated);
                        reply_msg = format!("Rotated shape by {} degrees.", d);
                    }
                    else
                    {
                        reply_msg = "No shape to rotate.".to_string();
                    }
                }
                else
                {
                    reply_msg = "Invalid rotation degrees.".to_string();
                }
            }
            ["print"] => {
                reply_msg = "Current shapes:".to_string();
            }
            ["types"] => {
                let shape_types: Vec<String> = shapes.iter()
                    .map(|shape| shape.shape_type())
                    .collect();

                reply_msg = format!("Shape types: {}", shape_types.join(", "));
            }
            _ => {
                reply_msg = format!("Unknown command: {}", command);
            }
        }

        let shape_descriptions: Vec<String> = shapes.iter()
            .map(|shape| shape.Tostring())
            .collect();
        let shape_types: Vec<String> = shapes.iter()
            .map(|shape| shape.shape_type())
            .collect();

        let reply = SayResponse {
            message: reply_msg,
            shapes: shape_descriptions,
            types: shape_types,
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

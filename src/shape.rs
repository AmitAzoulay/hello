pub trait Shape{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn getShape(&self) -> str;

}

pub struct Circle{
    radius: f64,
}
impl Circle{
    fn new(radius: f64) -> Circle{
        Circle{radius: radius}
    }
}
impl Shape for Circle{
    fn area(&self) -> f64{
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64{
        2.0 * PI * self.radius
    }
}

pub struct Rectangle{
    width: f64,
    height: f64,
}
impl Rectangle{
    fn new(width: f64, height: f64) -> Rectangle{
        Rectangle{width: width, height: height}
    }
}
impl Shape for Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
    fn perimeter(&self) -> f64{
        self.width * self.height * 2.0
    }
}
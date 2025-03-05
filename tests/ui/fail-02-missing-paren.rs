use enum_inner_method::enum_inner_method;

pub struct Circle;
impl Circle {
    pub fn area(&self) -> f64 {
        3.14
    }
}

pub struct Rectangle;
impl Rectangle {
    pub fn area(&self) -> f64 {
        6.0
    }
}

#[enum_inner_method(area -> f64)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
}

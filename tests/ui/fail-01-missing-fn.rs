use enum_inner_method::enum_inner_method;

pub struct Circle;
impl Circle {
   // Note: Doesn't implement area
}

pub struct Rectangle;
impl Rectangle {
    pub fn area(&self, x: f64) -> f64 {
        6.0 * x
    }
}

#[enum_inner_method(area(f64) -> f64)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
}

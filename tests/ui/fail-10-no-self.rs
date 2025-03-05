
use enum_inner_method::enum_inner_method;

pub struct Circle;
impl Circle {
    pub fn nothing() {}
}

pub struct Rectangle;
impl Rectangle {
    pub fn nothing() {}
}

#[enum_inner_method(fn nothing())]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
    let shapes = [ShapeEnum::Circle(Circle), ShapeEnum::Rectangle(Rectangle)];
}

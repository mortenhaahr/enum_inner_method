use enum_inner_method::enum_inner_method;

pub struct Circle;
impl Circle {
    pub fn nothing(&self) {
    }
}

pub struct Rectangle;
impl Rectangle {
    pub fn nothing(&self) {
    }
}

#[enum_inner_method(fn nothing(&self))]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
    let shapes = [ShapeEnum::Circle(Circle), ShapeEnum::Rectangle(Rectangle)];
    shapes.iter().for_each(|shape| shape.nothing());
}

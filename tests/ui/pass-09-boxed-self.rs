use enum_inner_method::enum_inner_method;

pub trait Shape {
    fn area(&self, x: f64, y: f64) -> f64 {
        x*y
    }
}

pub struct Circle;
impl Shape for Circle {}

pub struct Rectangle;
impl Shape for Rectangle {}

#[enum_inner_method(area(f64, f64) -> f64)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
    Me(Box<Self>),
}

fn main() {
    let circle = ShapeEnum::Circle(Circle);
    let rectangle = ShapeEnum::Rectangle(Rectangle);
    let nested = ShapeEnum::Me(Box::new(ShapeEnum::Circle(Circle)));

    assert_eq!(circle.area(2.0, 1.0), 2.0);
    assert_eq!(rectangle.area(4.0, 1.0), 4.0);
    assert_eq!(nested.area(2.0, 1.0), 2.0);
}

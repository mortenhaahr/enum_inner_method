use enum_inner_method::enum_inner_method;

trait Shape {
    fn area(&self, x: f64, y: f64) -> f64 {
        x*y
    }
}

pub struct Circle;
impl Shape for Circle {}

pub struct Rectangle;
impl Shape for Rectangle {}

#[enum_inner_method(fn area(&self, x: f64, y: f64) -> f64)]
pub enum ShapeEnum {
    Circle(Box<Circle>),
    Rectangle(Box<Rectangle>),
}

fn main() {
    let circle = ShapeEnum::Circle(Box::new(Circle));
    let rectangle = ShapeEnum::Rectangle(Box::new(Rectangle));

    assert_eq!(circle.area(2.0, 1.0), 2.0);
    assert_eq!(rectangle.area(4.0, 1.0), 4.0);
}

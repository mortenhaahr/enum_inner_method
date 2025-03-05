use enum_inner_method::enum_inner_method;

trait Shape {
    fn area(&self, x: f64, y: f64) -> f64;
}

pub struct Circle;
impl Shape for Circle {
    fn area(&self, x: f64, y: f64) -> f64 {
        3.14 * x * y
    }
}

pub struct Rectangle;
impl Shape for Rectangle {
    fn area(&self, x: f64, y: f64) -> f64 {
        6.0 * x * y
    }
}

#[enum_inner_method(area(f64, f64) -> f64)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
    let circle = ShapeEnum::Circle(Circle);
    let rectangle = ShapeEnum::Rectangle(Rectangle);

    assert_eq!(circle.area(2.0, 1.0), 6.28);
    assert_eq!(rectangle.area(4.0, 1.0), 24.0);
}

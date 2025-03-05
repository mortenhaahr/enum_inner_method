use enum_inner_method::enum_inner_method;

pub struct Circle;
impl Circle {
    pub fn area(&self, x: f64) -> f64 {
        3.14 * x
    }
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
    let circle = ShapeEnum::Circle(Circle);
    let rectangle = ShapeEnum::Rectangle(Rectangle);

    assert_eq!(circle.area(2.0), 6.28);
    assert_eq!(rectangle.area(4.0), 24.0);
}

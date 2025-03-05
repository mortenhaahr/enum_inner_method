use enum_inner_method::enum_inner_method;

pub struct Circle {
    radius: f64,
}
impl Circle {
    pub fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[enum_inner_method(fn area(&self) -> f64)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
    let circle = ShapeEnum::Circle(Circle{radius: 1.0});
    let rectangle = ShapeEnum::Rectangle(Rectangle{width: 2.0, height: 3.0});

    assert_eq!(circle.area(), 3.14);
    assert_eq!(rectangle.area(), 6.0);
}

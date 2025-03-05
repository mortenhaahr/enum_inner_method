use enum_inner_method::enum_inner_method;

#[derive(Debug, PartialEq)]
pub struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq)]
pub struct Drawable { point: Point }

pub struct Circle;
impl Circle {
    pub fn draw(&self, point: Point) -> Drawable {
        Drawable { point }
    }
}

pub struct Rectangle;
impl Rectangle {
    pub fn draw(&self, point: Point) -> Drawable {
        Drawable { point }
    }
}

#[enum_inner_method(fn draw(&self, x: Point) -> Drawable)]
pub enum ShapeEnum {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn main() {
    let shapes = [ShapeEnum::Circle(Circle), ShapeEnum::Rectangle(Rectangle)];
    let res0 = Drawable{point: Point{x: 1.0, y: 1.0}};
    let res1 = Drawable{point: Point{x: 1.0, y: 1.0}};
    assert_eq!(shapes[0].draw(Point{x: 1.0, y: 1.0}), res0);
    assert_eq!(shapes[1].draw(Point{x: 1.0, y: 1.0}), res1);
}

use enum_inner_method::enum_inner_method;

pub struct Plus {
    pub res: i64,
}
impl Plus {
    pub fn calc(&self, x: i64, y: i64) -> i64 {
        x + y
    }
    pub fn res(&self) -> i64 {
        self.res
    }
}

pub struct Minus {
    pub res: i64,
}
impl Minus {
    pub fn calc(&self, x: i64, y: i64) -> i64{
        x - y
    }
    pub fn res(&self) -> i64 {
        self.res
    }
}

#[enum_inner_method(calc(i64, i64) -> i64)]
#[enum_inner_method(res() -> i64)]
pub enum BinOp{
    Plus(Plus),
    Minus(Minus),
}

fn main() {
    let plus = BinOp::Plus(Plus{res: 0});
    let minus = BinOp::Minus(Minus{res: 0});

    assert_eq!(plus.calc(1, 2), 3);
    assert_eq!(minus.calc(1, 2), -1);
}

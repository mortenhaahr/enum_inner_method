use enum_inner_method::enum_inner_method;

pub struct Plus {
    pub res: i64,
}
impl Plus {
    pub fn calc(&mut self, x: i64, y: i64) {
        self.res = x + y;
    }
    pub fn res(&self) -> i64 {
        self.res
    }
}

pub struct Minus {
    pub res: i64,
}
impl Minus {
    pub fn calc(&mut self, x: i64, y: i64) {
        self.res = x - y;
    }
    pub fn res(&self) -> i64 {
        self.res
    }
}

#[enum_inner_method(fn calc(&mut self, x: i64, y: i64))]
#[enum_inner_method(fn res(&self) -> i64)]
pub enum BinOp {
    Plus(Plus),
    Minus(Minus),
}

fn main() {
    let mut plus = BinOp::Plus(Plus { res: 0 });
    let mut minus = BinOp::Minus(Minus { res: 0 });

    plus.calc(1, 2);
    minus.calc(1, 2);

    assert_eq!(plus.res(), 3);
    assert_eq!(minus.res(), -1);
}

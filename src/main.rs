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

    pub fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
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

    pub fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

#[enum_inner_method(fn calc(&mut self, x: i64, y: i64))]
#[enum_inner_method(fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str)]
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
    assert_eq!(plus.lifetime("a", "ab"), "ab");
    assert_eq!(minus.lifetime("abc", "ab"), "abc");
}

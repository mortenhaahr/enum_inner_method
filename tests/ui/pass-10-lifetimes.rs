use enum_inner_method::enum_inner_method;

pub struct Plus {}
impl Plus {
    pub fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

pub struct Minus {}
impl Minus {
    pub fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

#[enum_inner_method(fn lifetime<'a>(&self, x: &'a str, y: &'a str) -> &'a str)]
pub enum BinOp {
    Plus(Plus),
    Minus(Minus),
}

fn main() {
    let plus = BinOp::Plus(Plus { });
    let minus = BinOp::Minus(Minus { });

    assert_eq!(plus.lifetime("a", "ab"), "ab");
    assert_eq!(minus.lifetime("abc", "ab"), "abc");
}

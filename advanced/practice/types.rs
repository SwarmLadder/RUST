#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
impl Expr {
    pub fn lit(v: i32) -> Self {
        Self::Lit(v)
    }
    pub fn sum(a: Self, b: Self) -> Self {
        Self::Add(Box::new(a), Box::new(b))
    }
    pub fn product(a: Self, b: Self) -> Self {
        Self::Mul(Box::new(a), Box::new(b))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revision {
    pub version: u32,
    pub value: i32,
}

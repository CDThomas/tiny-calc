#[derive(Debug, PartialEq)]
pub struct Calculation {
    pub expr: Expr,
    pub formatter: Option<Formatter>,
}

#[derive(Debug, PartialEq)]
pub struct Formatter {}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(Integer),
    BinaryOp(BinaryOp),
}

#[derive(Debug, PartialEq)]
pub struct Integer {
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: Op,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
}

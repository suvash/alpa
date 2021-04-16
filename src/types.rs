#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Operation(Operation),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Int(i32),
    Sym(Symbol),
    SExpr(Vec<Box<Expr>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivideByZero,
}

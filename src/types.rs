use crate::sankhya::Sankhya;

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
    Num(Sankhya),
    Sym(Symbol),
    SExpr(Vec<Box<Expr>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivideByZero,
}

use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(x) => write!(f, "{}", x),
            _ => write!(f, "printer not implemented yet"),
        }
    }
}

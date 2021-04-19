use std::fmt;

use crate::ntypes::Sankhya;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
        }
    }
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
    QExpr(Vec<Box<Expr>>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Sym(s) => match s {
                Symbol::Operation(o) => write!(f, "{}", o),
            },
            Expr::SExpr(sexpr) => {
                write!(
                    f,
                    "({})",
                    sexpr
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Expr::QExpr(qexpr) => {
                write!(
                    f,
                    "'({})",
                    qexpr
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivideByZero(Sankhya, Sankhya),
    NotANumberOperation(Expr),
    NotANumber(Expr),
}

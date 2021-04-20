use std::fmt;

use crate::ntypes::Sankhya;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NumberOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for NumberOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberOp::Add => write!(f, "+"),
            NumberOp::Subtract => write!(f, "-"),
            NumberOp::Multiply => write!(f, "*"),
            NumberOp::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum QExprOp {
    First,
    Rest,
    Eval,
}

impl fmt::Display for QExprOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QExprOp::First => write!(f, "पहिलो"),
            QExprOp::Rest => write!(f, "बाँकी"),
            QExprOp::Eval => write!(f, "बिस्तार"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExprOp {
    Quote,
}

impl fmt::Display for SExprOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExprOp::Quote => write!(f, "उद्धरण"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    NumberOp(NumberOp),
    QExprOp(QExprOp),
    SExprOp(SExprOp),
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
                Symbol::NumberOp(o) => write!(f, "{}", o),
                Symbol::QExprOp(o) => write!(f, "{}", o),
                Symbol::SExprOp(o) => write!(f, "{}", o),
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
    InvalidNumberOfQExprArguments(QExprOp, usize),
    InvalidNumberOfSExprArguments(SExprOp, usize),
    EmptyQExpr(Expr),
    InvalidOp(Expr),
    NotANumber(Expr),
    NotAQExpr(Expr),
    NotASExpr(Expr),
}

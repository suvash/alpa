use std::fmt;

use crate::core::CoreFn;
use crate::ntypes::Sankhya;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum NumOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for NumOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOp::Add => write!(f, "+"),
            NumOp::Subtract => write!(f, "-"),
            NumOp::Multiply => write!(f, "*"),
            NumOp::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum QExprOp {
    First,
    Rest,
    Len,
    Eval,
}

impl fmt::Display for QExprOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QExprOp::First => write!(f, "पहिलो"),
            QExprOp::Rest => write!(f, "बाँकी"),
            QExprOp::Len => write!(f, "वटा"),
            QExprOp::Eval => write!(f, "बिस्तार"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum QExprsOp {
    Cons,
    Join,
    Def,
}

impl fmt::Display for QExprsOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QExprsOp::Cons => write!(f, "निर्माण"),
            QExprsOp::Join => write!(f, "एकत्र"),
            QExprsOp::Def => write!(f, "नामक"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SExprOp {
    Quote,
    PrintEnv,
}

impl fmt::Display for SExprOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExprOp::Quote => write!(f, "उद्धरण"),
            SExprOp::PrintEnv => write!(f, "वातावरण"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Symbol {
    Identifier(String),
    NumOp(NumOp),
    QExprOp(QExprOp),
    QExprsOp(QExprsOp),
    SExprOp(SExprOp),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Identifier(s) => write!(f, "आइडेन्टिफायर({})", s),
            Symbol::NumOp(o) => write!(f, "नम-अप({})", o),
            Symbol::QExprOp(o) => write!(f, "क्यु-एक्सपर्-अप({})", o),
            Symbol::QExprsOp(o) => write!(f, "क्यु-एक्सपर्स-अप({})", o),
            Symbol::SExprOp(o) => write!(f, "एस्-एक्सपर्-अप({})", o),
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Core(Symbol, CoreFn),
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Core(sym, _) => write!(f, "Core({:?}, pointer)", sym),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        use Function::*;
        match (self, other) {
            (Core(self_sym, _), Core(other_sym, _)) => self_sym == other_sym,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Core(sym, _) => write!(f, "कोर({}, प्वाइन्टर)", sym),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(Sankhya),
    Sym(Symbol),
    SExpr(Vec<Box<Expr>>),
    QExpr(Vec<Box<Expr>>),
    Fun(Function),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Sym(s) => match s {
                Symbol::Identifier(s) => write!(f, "{}", s),
                Symbol::NumOp(o) => write!(f, "{}", o),
                Symbol::QExprOp(o) => write!(f, "{}", o),
                Symbol::QExprsOp(o) => write!(f, "{}", o),
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
            Expr::Fun(fun) => {
                write!(f, "{}", fun)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivideByZero(Sankhya, Sankhya),
    InvalidNumberOfNumArguments(NumOp, usize),
    InvalidNumberOfQExprArguments(QExprOp, usize),
    InvalidNumberOfQExprsArguments(QExprsOp, usize),
    InvalidNumberOfSExprArguments(SExprOp, usize),
    EmptyQExpr(Expr),
    InvalidOp(Expr),
    NotANumber(Expr),
    NotASymbol(Expr),
    NotAQExpr(Expr),
    NotASExpr(Expr),
    UnboundSymbol(Symbol),
    UnEqualDefList(Expr, Vec<Box<Expr>>),
}

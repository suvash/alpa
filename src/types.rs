use std::collections::HashMap;
use std::fmt;

use crate::core::CoreFn;
use crate::ntypes::Sankhya;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ExprsOp {
    Equal,
    NotEqual,
    If,
    Import,
    Print,
    Error,
}

impl fmt::Display for ExprsOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprsOp::Equal => write!(f, "=="),
            ExprsOp::NotEqual => write!(f, "!="),
            ExprsOp::If => write!(f, "यदि"),
            ExprsOp::Import => write!(f, "आयात"),
            ExprsOp::Print => write!(f, "छाप"),
            ExprsOp::Error => write!(f, "समस्या"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum NumOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl fmt::Display for NumOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOp::Add => write!(f, "+"),
            NumOp::Subtract => write!(f, "-"),
            NumOp::Multiply => write!(f, "*"),
            NumOp::Divide => write!(f, "/"),
            NumOp::GreaterThan => write!(f, ">"),
            NumOp::GreaterThanOrEqual => write!(f, ">="),
            NumOp::LessThan => write!(f, "<"),
            NumOp::LessThanOrEqual => write!(f, "<="),
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
    Put,
    Lambda,
}

impl fmt::Display for QExprsOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QExprsOp::Cons => write!(f, "निर्माण"),
            QExprsOp::Join => write!(f, "एकत्र"),
            QExprsOp::Def => write!(f, "नामक"),
            QExprsOp::Put => write!(f, "="),
            QExprsOp::Lambda => write!(f, "ल्याम्बडा"),
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
pub struct Boolean(pub bool);

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match self.0 {
            true => "सत्य",
            false => "गलत",
        };

        write!(f, "बुलियन({})", formatted)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Symbol {
    ExprsOp(ExprsOp),
    NumOp(NumOp),
    QExprOp(QExprOp),
    QExprsOp(QExprsOp),
    SExprOp(SExprOp),
    Identifier(String),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::ExprsOp(o) => write!(f, "एक्सपर्स-अप({})", o),
            Symbol::NumOp(o) => write!(f, "नम-अप({})", o),
            Symbol::QExprOp(o) => write!(f, "क्यु-एक्सपर्-अप({})", o),
            Symbol::QExprsOp(o) => write!(f, "क्यु-एक्सपर्स-अप({})", o),
            Symbol::SExprOp(o) => write!(f, "एस्-एक्सपर्-अप({})", o),
            Symbol::Identifier(s) => write!(f, "आइडेन्टिफायर({})", s),
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Core(Symbol, CoreFn),
    Lambda(Vec<Symbol>, Box<Expr>, HashMap<Symbol, Expr>),
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Core(sym, _) => write!(f, "Core({:?}, pointer)", sym),
            Function::Lambda(syms, expr, hmap) => {
                write!(
                    f,
                    "Lambda(syms : {:?}, body : {:?}, env : {:?})",
                    syms, expr, hmap
                )
            }
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        use Function::*;
        match (self, other) {
            (Core(self_sym, _), Core(other_sym, _)) => self_sym == other_sym,
            (Lambda(s_syms, s_expr, s_hmap), Lambda(o_syms, o_expr, o_hmap)) => {
                s_syms == o_syms && s_expr == o_expr && s_hmap == o_hmap
            }
            _ => false,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Core(sym, _) => write!(f, "कोर({}, प्वाइन्टर)", sym),
            Function::Lambda(syms, expr, hmap) => {
                write!(
                    f,
                    "ल्याम्बडा(सिम्बलहरु : {:?}, बडी : {:?}, वातावरण : {:?})",
                    syms, expr, hmap
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Bool(Boolean),
    Num(Sankhya),
    Str(String),
    Sym(Symbol),
    SExpr(Vec<Box<Expr>>),
    QExpr(Vec<Box<Expr>>),
    Fun(Function),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Str(s) => write!(f, "\"{}\"", s),
            Expr::Sym(s) => match s {
                Symbol::ExprsOp(o) => write!(f, "{}", o),
                Symbol::NumOp(o) => write!(f, "{}", o),
                Symbol::QExprOp(o) => write!(f, "{}", o),
                Symbol::QExprsOp(o) => write!(f, "{}", o),
                Symbol::SExprOp(o) => write!(f, "{}", o),
                Symbol::Identifier(s) => write!(f, "{}", s),
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

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    DivideByZero(Sankhya, Sankhya),
    InvalidNumberOfNumArguments(NumOp, usize),
    InvalidNumberOfExprsArguments(ExprsOp, usize),
    InvalidNumberOfQExprArguments(QExprOp, usize),
    InvalidNumberOfQExprsArguments(QExprsOp, usize),
    InvalidNumberOfSExprArguments(SExprOp, usize),
    EmptyQExpr(Expr),
    ParseError(String),
    ImportError(Expr),
    InvalidOp(Expr),
    NotABoolean(Expr),
    NotANumber(Expr),
    NotASymbol(Expr),
    NotAString(Expr),
    NotAnIdentifier(Expr),
    NotAQExpr(Expr),
    NotASExpr(Expr),
    UnboundSymbol(Symbol),
    UnEqualDefList(Expr, Vec<Box<Expr>>),
    ThrowError(String),
}

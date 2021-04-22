use crate::environment::Env;
use crate::evaluator;
use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, NumOp, QExprOp, QExprsOp, SExprOp};

pub fn nums_add(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match evaluator::eval(env, &**first)? {
            Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                match (a?, evaluator::eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 + y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }
            }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(NumOp::Add, exprs.len())),
    }
}

pub fn nums_subtract(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match evaluator::eval(env, &**first)? {
            Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                match (a?, evaluator::eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 - y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }
            }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Subtract,
            exprs.len(),
        )),
    }
}

pub fn nums_multiply(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match evaluator::eval(env, &**first)? {
            Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                match (a?, evaluator::eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 * y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }
            }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Multiply,
            exprs.len(),
        )),
    }
}

pub fn nums_divide(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match evaluator::eval(env, &**first)? {
            Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                match (a?, evaluator::eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => match y.0 {
                        0 => Err(Error::DivideByZero(x, y)),
                        _ => Ok(Expr::Num(Sankhya(x.0 / y.0))),
                    },
                    (_, y) => Err(Error::NotANumber(y)),
                }
            }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Divide,
            exprs.len(),
        )),
    }
}

pub fn qexpr_first(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match evaluator::eval(env, &expr)? {
            Expr::QExpr(qexpr) => match qexpr.split_first() {
                Some((first, _)) => Ok(Expr::QExpr(vec![first.clone()])),
                None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
            },
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::First,
            exprs.len(),
        )),
    }
}

pub fn qexpr_rest(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match evaluator::eval(env, &expr)? {
            Expr::QExpr(qexpr) => match qexpr.split_first() {
                Some((_, rest)) => Ok(Expr::QExpr(rest.to_vec())),
                None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
            },
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::Rest,
            exprs.len(),
        )),
    }
}

pub fn qexpr_len(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match evaluator::eval(env, &expr)? {
            Expr::QExpr(qexpr) => Ok(Expr::Num(Sankhya(qexpr.len() as i32))),
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::Len,
            exprs.len(),
        )),
    }
}

pub fn qexpr_eval(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match evaluator::eval(env, &expr)? {
            Expr::QExpr(qexpr) => evaluator::eval(env, &Expr::SExpr(qexpr.to_vec())),
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::Eval,
            exprs.len(),
        )),
    }
}

pub fn qexprs_cons(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [pref_expr, expr] => match evaluator::eval(env, &**expr)? {
            Expr::QExpr(_) => {
                let first = Box::new(Expr::QExpr(vec![pref_expr.clone()]));
                qexprs_join(env, &[first, expr.clone()])
            }
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprsArguments(
            QExprsOp::Cons,
            exprs.len(),
        )),
    }
}

pub fn qexprs_join(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfQExprsArguments(
            QExprsOp::Join,
            exprs.len(),
        )),
        _ => exprs.iter().fold(Ok(Expr::QExpr(vec![])), |a, b| {
            match (a?, evaluator::eval(env, b)?) {
                (Expr::QExpr(mut x), Expr::QExpr(y)) => {
                    x.extend(y);
                    Ok(Expr::QExpr(x))
                }
                (_, y) => Err(Error::NotAQExpr(y)),
            }
        }),
    }
}

pub fn sexpr_quote(exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfSExprArguments(
            SExprOp::Quote,
            exprs.len(),
        )),
        _ => Ok(Expr::QExpr(exprs.to_vec())),
    }
}

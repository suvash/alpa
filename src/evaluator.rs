use crate::env::Env;
use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

fn nums_add(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match eval(env, &**first)? {
            Expr::Num(s) => rest
                .iter()
                .fold(Ok(Expr::Num(s)), |a, b| match (a?, eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 + y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Add,
            exprs.len(),
        )),
    }
}

fn nums_subtract(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match eval(env, &**first)? {
            Expr::Num(s) => rest
                .iter()
                .fold(Ok(Expr::Num(s)), |a, b| match (a?, eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 - y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Subtract,
            exprs.len(),
        )),
    }
}

fn nums_multiply(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match eval(env, &**first)? {
            Expr::Num(s) => rest
                .iter()
                .fold(Ok(Expr::Num(s)), |a, b| match (a?, eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => Ok(Expr::Num(Sankhya(x.0 * y.0))),
                    (_, y) => Err(Error::NotANumber(y)),
                }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Multiply,
            exprs.len(),
        )),
    }
}

fn nums_divide(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [first, rest @ ..] => match eval(env, &**first)? {
            Expr::Num(s) => rest
                .iter()
                .fold(Ok(Expr::Num(s)), |a, b| match (a?, eval(env, b)?) {
                    (Expr::Num(x), Expr::Num(y)) => match y.0 {
                        0 => Err(Error::DivideByZero(x, y)),
                        _ => Ok(Expr::Num(Sankhya(x.0 / y.0))),
                    },
                    (_, y) => Err(Error::NotANumber(y)),
                }),
            x => Err(Error::NotANumber(x)),
        },
        _ => Err(Error::InvalidNumberOfNumArguments(
            NumOp::Divide,
            exprs.len(),
        )),
    }
}

fn qexpr_first(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match eval(env, &expr)? {
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

fn qexpr_rest(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match eval(env, &expr)? {
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

fn qexpr_len(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match eval(env, &expr)? {
            Expr::QExpr(qexpr) => Ok(Expr::Num(Sankhya(qexpr.len() as i32))),
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::Len,
            exprs.len(),
        )),
    }
}

fn qexpr_eval(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match eval(env, &expr)? {
            Expr::QExpr(qexpr) => eval(env, &Expr::SExpr(qexpr.to_vec())),
            x => Err(Error::NotAQExpr(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            QExprOp::Eval,
            exprs.len(),
        )),
    }
}

fn qexprs_cons(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [pref_expr, expr] => match eval(env, &**expr)? {
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

fn qexprs_join(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfQExprsArguments(
            QExprsOp::Join,
            exprs.len(),
        )),
        _ => exprs
            .iter()
            .fold(Ok(Expr::QExpr(vec![])), |a, b| match (a?, eval(env, b)?) {
                (Expr::QExpr(mut x), Expr::QExpr(y)) => {
                    x.extend(y);
                    Ok(Expr::QExpr(x))
                }
                (_, y) => Err(Error::NotAQExpr(y)),
            }),
    }
}

fn sexpr_quote(exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfSExprArguments(
            SExprOp::Quote,
            exprs.len(),
        )),
        _ => Ok(Expr::QExpr(exprs.to_vec())),
    }
}

pub fn eval(env: &Env, expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(sym) => match env.get(&sym) {
            Some(env_expr) => Ok(env_expr.clone()),
            None => Ok(expr.clone()),
        },
        Expr::SExpr(sexpr) => match &**sexpr {
            [] => Ok(expr.clone()),
            [expr] => eval(env, expr),
            [oper, exprs @ ..] => match &**oper {
                Expr::Sym(Symbol::NumOp(NumOp::Add)) => nums_add(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Subtract)) => nums_subtract(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Multiply)) => nums_multiply(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Divide)) => nums_divide(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::First)) => qexpr_first(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Rest)) => qexpr_rest(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Len)) => qexpr_len(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Eval)) => qexpr_eval(env, exprs),
                Expr::Sym(Symbol::QExprsOp(QExprsOp::Cons)) => qexprs_cons(env, exprs),
                Expr::Sym(Symbol::QExprsOp(QExprsOp::Join)) => qexprs_join(env, exprs),
                Expr::Sym(Symbol::SExprOp(SExprOp::Quote)) => sexpr_quote(exprs),
                x => Err(Error::InvalidOp(x.clone())),
            },
        },
        Expr::QExpr(_) => Ok(expr.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_eval_success() {
        let input = Expr::SExpr(vec![
            Box::new(Expr::Sym(Symbol::NumOp(NumOp::Add))),
            Box::new(Expr::Num(Sankhya(2))),
            Box::new(Expr::Num(Sankhya(-5))),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Multiply))),
                Box::new(Expr::Num(Sankhya(4))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Divide))),
                Box::new(Expr::Num(Sankhya(-10))),
                Box::new(Expr::Num(Sankhya(2))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumOp(NumOp::Subtract))),
                Box::new(Expr::Num(Sankhya(-1))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
        ]);
        let env = Env::new(HashMap::new(), None);

        assert_eq!(eval(&env, &input), Ok(Expr::Num(Sankhya(6))));
    }
}

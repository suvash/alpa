use crate::environment::Env;
use crate::evaluator;
use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, NumOp, QExprOp, QExprsOp, SExprOp};

macro_rules! nums_fn {
    ($fn_name:ident, $op:expr, $x:ident, $y:ident, $x_y_body:block) => {
        pub fn $fn_name(env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
            match &exprs[..] {
                [first, rest @ ..] => match evaluator::eval(env, &**first)? {
                    Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                        match (a?, evaluator::eval(env, b)?) {
                            (Expr::Num($x), Expr::Num($y)) => $x_y_body,
                            (_, y) => Err(Error::NotANumber(y)),
                        }
                    }),
                    x => Err(Error::NotANumber(x)),
                },
                _ => Err(Error::InvalidNumberOfNumArguments($op, exprs.len())),
            }
        }
    };
}

nums_fn!(nums_add, NumOp::Add, x, y, {
    Ok(Expr::Num(Sankhya(x.0 + y.0)))
});

nums_fn!(nums_subtract, NumOp::Subtract, x, y, {
    Ok(Expr::Num(Sankhya(x.0 - y.0)))
});

nums_fn!(nums_multiply, NumOp::Multiply, x, y, {
    Ok(Expr::Num(Sankhya(x.0 * y.0)))
});

nums_fn!(nums_divide, NumOp::Divide, x, y, {
    match y.0 {
        0 => Err(Error::DivideByZero(x, y)),
        _ => Ok(Expr::Num(Sankhya(x.0 / y.0))),
    }
});

macro_rules! qexpr_fn {
    ($fn_name:ident, $op:expr, $env:ident, $qexpr:ident, $qexpr_body:block) => {
        pub fn $fn_name($env: &Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
            match &exprs[..] {
                [expr] => match evaluator::eval($env, &expr)? {
                    Expr::QExpr($qexpr) => $qexpr_body,
                    x => Err(Error::NotAQExpr(x.clone())),
                },
                _ => Err(Error::InvalidNumberOfQExprArguments(
                    QExprOp::First,
                    exprs.len(),
                )),
            }
        }
    };
}

qexpr_fn!(qexpr_first, QExprOp::First, env, qexpr, {
    match qexpr.split_first() {
        Some((first, _)) => Ok(Expr::QExpr(vec![first.clone()])),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
});

qexpr_fn!(qexpr_rest, QExprOp::Rest, env, qexpr, {
    match qexpr.split_first() {
        Some((_, rest)) => Ok(Expr::QExpr(rest.to_vec())),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
});

qexpr_fn!(qexpr_len, QExprOp::Len, env, qexpr, {
    Ok(Expr::Num(Sankhya(qexpr.len() as i32)))
});

qexpr_fn!(qexpr_eval, QExprOp::Eval, env, qexpr, {
    evaluator::eval(env, &Expr::SExpr(qexpr.to_vec()))
});

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

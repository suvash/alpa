use crate::env::Env;
use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, NumberOp, QExprOp, QExprsOp, SExprOp, Symbol};

fn num_add(sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    Ok(Expr::Num(Sankhya(sx.0 + sy.0)))
}

fn num_subtract(sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    Ok(Expr::Num(Sankhya(sx.0 - sy.0)))
}

fn num_multiply(sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    Ok(Expr::Num(Sankhya(sx.0 * sy.0)))
}

fn num_divide(sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    match sy.0 {
        0 => Err(Error::DivideByZero(sx, sy)),
        _ => Ok(Expr::Num(Sankhya(sx.0 / sy.0))),
    }
}

fn num_oper(oper: &NumberOp, sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    match oper {
        NumberOp::Add => num_add(sx, sy),
        NumberOp::Subtract => num_subtract(sx, sy),
        NumberOp::Multiply => num_multiply(sx, sy),
        NumberOp::Divide => num_divide(sx, sy),
    }
}

fn nums_oper_args(env: &Env, oper: &NumberOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    let begin = eval(env, &args[0])?;

    match begin {
        Expr::Num(_) => args[1..]
            .iter()
            .fold(Ok(begin), |a, b| match (a?, eval(env, b)?) {
                (Expr::Num(x), Expr::Num(y)) => num_oper(oper, x, y),
                (_, y) => Err(Error::NotANumber(y)),
            }),
        x => Err(Error::NotANumber(x)),
    }
}

fn qexpr_first(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match qexpr.split_first() {
        Some((first, _)) => Ok(Expr::QExpr(vec![first.clone()])),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
}

fn qexpr_rest(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match qexpr.split_first() {
        Some((_, rest)) => Ok(Expr::QExpr(rest.to_vec())),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
}

fn qexpr_len(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    Ok(Expr::Num(Sankhya(qexpr.len() as i32)))
}

fn qexpr_eval(env: &Env, qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    eval(env, &Expr::SExpr(qexpr.to_vec()))
}

fn qexpr_oper(env: &Env, oper: &QExprOp, qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match oper {
        QExprOp::First => qexpr_first(qexpr),
        QExprOp::Rest => qexpr_rest(qexpr),
        QExprOp::Len => qexpr_len(qexpr),
        QExprOp::Eval => qexpr_eval(env, qexpr),
    }
}

fn qexpr_oper_args(env: &Env, oper: &QExprOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    match &args[..] {
        [arg] => match eval(env, &arg)? {
            Expr::QExpr(qexpr) => qexpr_oper(env, oper, &qexpr),
            expr => Err(Error::NotAQExpr(expr.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            oper.clone(),
            args.len(),
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

fn qexprs_join(env: &Env, qexprs: &[Box<Expr>]) -> Result<Expr, Error> {
    qexprs
        .iter()
        .fold(Ok(Expr::QExpr(vec![])), |a, b| match (a?, eval(env, b)?) {
            (Expr::QExpr(mut x), Expr::QExpr(y)) => {
                x.extend(y);
                Ok(Expr::QExpr(x))
            }
            (_, y) => Err(Error::NotAQExpr(y)),
        })
}

fn qexprs_oper(env: &Env, oper: &QExprsOp, qexprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match oper {
        QExprsOp::Cons => qexprs_cons(env, &qexprs),
        QExprsOp::Join => qexprs_join(&env, qexprs),
    }
}

fn qexprs_oper_args(env: &Env, oper: &QExprsOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    match &args[..] {
        [] => Err(Error::InvalidNumberOfQExprsArguments(
            oper.clone(),
            args.len(),
        )),
        _ => qexprs_oper(env, oper, &args),
    }
}

fn sexpr_quote(sexpr: &[Box<Expr>]) -> Result<Expr, Error> {
    Ok(Expr::QExpr(sexpr.to_vec()))
}

fn sexpr_oper(oper: &SExprOp, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match oper {
        SExprOp::Quote => sexpr_quote(exprs),
    }
}

fn sexpr_oper_args(oper: &SExprOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    match &args[..] {
        [] => Err(Error::InvalidNumberOfSExprArguments(
            oper.clone(),
            args.len(),
        )),
        _ => sexpr_oper(oper, &args),
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
            [oper, args @ ..] => match &**oper {
                Expr::Sym(Symbol::NumberOp(op)) => nums_oper_args(env, op, args),
                Expr::Sym(Symbol::QExprOp(op)) => qexpr_oper_args(env, op, args),
                Expr::Sym(Symbol::QExprsOp(op)) => qexprs_oper_args(env, op, args),
                Expr::Sym(Symbol::SExprOp(op)) => sexpr_oper_args(op, args),
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
            Box::new(Expr::Sym(Symbol::NumberOp(NumberOp::Add))),
            Box::new(Expr::Num(Sankhya(2))),
            Box::new(Expr::Num(Sankhya(-5))),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumberOp(NumberOp::Multiply))),
                Box::new(Expr::Num(Sankhya(4))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumberOp(NumberOp::Divide))),
                Box::new(Expr::Num(Sankhya(-10))),
                Box::new(Expr::Num(Sankhya(2))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::NumberOp(NumberOp::Subtract))),
                Box::new(Expr::Num(Sankhya(-1))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
        ]);
        let env = Env::new(HashMap::new(), None);

        assert_eq!(eval(&env, &input), Ok(Expr::Num(Sankhya(6))));
    }
}

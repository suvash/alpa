use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, NumberOp, QExprOp, SExprOp, Symbol};

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

fn num_oper_args(oper: &NumberOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    let begin = eval(&args[0])?;

    match begin {
        Expr::Num(_) => args[1..]
            .iter()
            .fold(Ok(begin), |a, b| match (a?, eval(b)?) {
                (Expr::Num(x), Expr::Num(y)) => num_oper(oper, x, y),
                (_, y) => Err(Error::NotANumber(y)),
            }),
        x => Err(Error::NotANumber(x)),
    }
}

fn qexpr_first(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match qexpr.split_first() {
        Some((first, _)) => Ok(*first.clone()),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
}

fn qexpr_rest(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match qexpr.split_first() {
        Some((_, rest)) => Ok(Expr::QExpr(rest.to_vec())),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
}

fn qexpr_eval(qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    eval(&Expr::SExpr(qexpr.to_vec()))
}

fn qexpr_oper(oper: &QExprOp, qexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match oper {
        QExprOp::First => qexpr_first(qexpr),
        QExprOp::Rest => qexpr_rest(qexpr),
        QExprOp::Eval => qexpr_eval(qexpr),
    }
}

fn qexpr_oper_args(oper: &QExprOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    match &args[..] {
        [arg] => match &**arg {
            Expr::QExpr(qexpr) => qexpr_oper(oper, &qexpr),
            expr => Err(Error::NotAQExpr(expr.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprArguments(
            oper.clone(),
            args.len(),
        )),
    }
}

fn sexpr_quote(sexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    Ok(Expr::QExpr(sexpr.clone()))
}

fn sexpr_oper(oper: &SExprOp, sexpr: &Vec<Box<Expr>>) -> Result<Expr, Error> {
    match oper {
        SExprOp::Quote => sexpr_quote(sexpr),
    }
}

fn sexpr_oper_args(oper: &SExprOp, args: &[Box<Expr>]) -> Result<Expr, Error> {
    match &args[..] {
        [arg] => match &**arg {
            Expr::SExpr(qexpr) => sexpr_oper(oper, &qexpr),
            expr => Err(Error::NotASExpr(expr.clone())),
        },
        _ => Err(Error::InvalidNumberOfSExprArguments(
            oper.clone(),
            args.len(),
        )),
    }
}

pub fn eval(expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(_) => Ok(expr.clone()),
        Expr::SExpr(sexpr) => match &**sexpr {
            [] => Ok(expr.clone()),
            [expr] => eval(expr),
            [oper, args @ ..] => match &**oper {
                Expr::Sym(Symbol::NumberOp(op)) => num_oper_args(&op, args),
                Expr::Sym(Symbol::QExprOp(op)) => qexpr_oper_args(&op, args),
                Expr::Sym(Symbol::SExprOp(op)) => sexpr_oper_args(&op, args),
                x => Err(Error::InvalidOp(x.clone())),
            },
        },
        Expr::QExpr(_) => Ok(expr.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(eval(&input), Ok(Expr::Num(Sankhya(6))));
    }
}

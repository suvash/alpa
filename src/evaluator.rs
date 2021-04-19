use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, Operation, Symbol};

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

fn num_oper(oper: &Operation, sx: Sankhya, sy: Sankhya) -> Result<Expr, Error> {
    match oper {
        Operation::Add => num_add(sx, sy),
        Operation::Subtract => num_subtract(sx, sy),
        Operation::Multiply => num_multiply(sx, sy),
        Operation::Divide => num_divide(sx, sy),
    }
}

fn num_oper_args(oper: &Operation, args: &[Box<Expr>]) -> Result<Expr, Error> {
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

pub fn eval(expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(_) => Ok(expr.clone()),
        Expr::SExpr(sexpr) => match sexpr.len() {
            0 => Ok(expr.clone()),
            1 => eval(&sexpr[0]),
            _ => {
                let first = eval(&sexpr[0])?;

                match &first {
                    Expr::Sym(Symbol::Operation(op)) => num_oper_args(&op, &sexpr[1..]),
                    x => Err(Error::NotANumberOperation(x.clone())),
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_success() {
        let input = Expr::SExpr(vec![
            Box::new(Expr::Sym(Symbol::Operation(Operation::Add))),
            Box::new(Expr::Num(Sankhya(2))),
            Box::new(Expr::Num(Sankhya(-5))),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Multiply))),
                Box::new(Expr::Num(Sankhya(4))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Divide))),
                Box::new(Expr::Num(Sankhya(-10))),
                Box::new(Expr::Num(Sankhya(2))),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Subtract))),
                Box::new(Expr::Num(Sankhya(-1))),
                Box::new(Expr::Num(Sankhya(5))),
            ])),
        ]);
        assert_eq!(eval(&input), Ok(Expr::Num(Sankhya(6))));
    }
}

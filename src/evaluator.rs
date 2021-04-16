use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, Operation, Symbol};

pub fn eval(expr: &Expr) -> Option<Expr> {
    match eval_expr(expr) {
        Ok(evaled) => Some(evaled),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

fn eval_expr(expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(_) => Ok(expr.clone()),
        Expr::SExpr(sexpr) => match sexpr.len() {
            0 => Ok(expr.clone()),
            1 => eval_expr(&sexpr[0]),
            _ => {
                let oper = eval_expr(&sexpr[0])?;
                let first = eval_expr(&sexpr[1]);

                sexpr[2..]
                    .iter()
                    .fold(first, |a, b| match (a?, eval_expr(b)?, &oper) {
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Add)),
                        ) => Ok(Expr::Num(Sankhya(x + y))),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Subtract)),
                        ) => Ok(Expr::Num(Sankhya(x - y))),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Multiply)),
                        ) => Ok(Expr::Num(Sankhya(x * y))),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Divide)),
                        ) => match y {
                            0 => Err(Error::DivideByZero),
                            _ => Ok(Expr::Num(Sankhya(x / y))),
                        },

                        _ => unreachable!(),
                    })
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
        assert_eq!(eval(&input), Some(Expr::Num(Sankhya(6))));
    }
}

use crate::sankhya::Sankhya;
use crate::types::{Expr, Operation, Symbol};

pub fn eval_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Num(_) => expr.clone(),
        Expr::Sym(_) => expr.clone(),
        Expr::SExpr(sexpr) => match sexpr.len() {
            0 => expr.clone(),
            1 => eval_expr(&sexpr[0]),
            _ => {
                let oper = &*sexpr[0];
                let first = eval_expr(&sexpr[1]);
                let rest = &sexpr[2..];

                rest.iter()
                    .fold(first, |a, b| match (a, eval_expr(&b), oper) {
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Add)),
                        ) => Expr::Num(Sankhya(x + y)),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Subtract)),
                        ) => Expr::Num(Sankhya(x - y)),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Multiply)),
                        ) => Expr::Num(Sankhya(x * y)),
                        (
                            Expr::Num(Sankhya(x)),
                            Expr::Num(Sankhya(y)),
                            Expr::Sym(Symbol::Operation(Operation::Divide)),
                        ) => Expr::Num(Sankhya(x / y)),

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
            Box::new(Expr::SExpr(vec![])),
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
        assert_eq!(eval_expr(&input), Expr::Num(Sankhya(6)));
    }
}

use crate::types::{Expr, Operation, Symbol};

pub fn eval_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Int(_) => expr.clone(),
        Expr::Sym(_) => expr.clone(),
        Expr::SExpr(sexpr) => match sexpr.len() {
            0 => Expr::SExpr(vec![]),
            1 => eval_expr(&sexpr[0]),
            _ => {
                let oper = &*sexpr[0];
                let first = eval_expr(&sexpr[1]);
                let rest = &sexpr[2..];

                rest.iter()
                    .fold(first, |a, b| match (a, eval_expr(&b), oper) {
                        (
                            Expr::Int(x),
                            Expr::Int(y),
                            Expr::Sym(Symbol::Operation(Operation::Add)),
                        ) => Expr::Int(x + y),
                        (
                            Expr::Int(x),
                            Expr::Int(y),
                            Expr::Sym(Symbol::Operation(Operation::Subtract)),
                        ) => Expr::Int(x - y),
                        (
                            Expr::Int(x),
                            Expr::Int(y),
                            Expr::Sym(Symbol::Operation(Operation::Multiply)),
                        ) => Expr::Int(x * y),
                        (
                            Expr::Int(x),
                            Expr::Int(y),
                            Expr::Sym(Symbol::Operation(Operation::Divide)),
                        ) => Expr::Int(x / y),

                        _ => unreachable!(),
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_success() {
        let input = Expr::SExpr(vec![
            Box::new(Expr::Sym(Symbol::Operation(Operation::Add))),
            Box::new(Expr::Int(2)),
            Box::new(Expr::Int(-5)),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Multiply))),
                Box::new(Expr::Int(4)),
                Box::new(Expr::Int(5)),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Divide))),
                Box::new(Expr::Int(-10)),
                Box::new(Expr::Int(2)),
            ])),
            Box::new(Expr::SExpr(vec![
                Box::new(Expr::Sym(Symbol::Operation(Operation::Subtract))),
                Box::new(Expr::Int(-1)),
                Box::new(Expr::Int(5)),
            ])),
        ]);
        assert_eq!(eval_expr(&input), Expr::Int(6));
    }


}

use crate::environment::Env;
use crate::types::{Error, Expr, Function};

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(sym) => env.lookup(&sym),
        Expr::SExpr(sexpr) => match &**sexpr {
            [] => Ok(expr.clone()),
            [expr] => eval(env, expr),
            [oper, exprs @ ..] => match eval(env, oper) {
                Ok(Expr::Fun(Function::Core(_, cf))) => cf(env, exprs),
                _ => Err(Error::InvalidOp(*oper.clone())),
            },
        },
        Expr::QExpr(_) => Ok(expr.clone()),
        Expr::Fun(_) => Ok(expr.clone()),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::ntypes::Sankhya;
    // use std::collections::HashMap;

    #[test]
    fn test_eval_success() {
        // let input = Expr::SExpr(vec![
        //     Box::new(Expr::Sym(Symbol::NumOp(NumOp::Add))),
        //     Box::new(Expr::Num(Sankhya(2))),
        //     Box::new(Expr::Num(Sankhya(-5))),
        //     Box::new(Expr::SExpr(vec![
        //         Box::new(Expr::Sym(Symbol::NumOp(NumOp::Multiply))),
        //         Box::new(Expr::Num(Sankhya(4))),
        //         Box::new(Expr::Num(Sankhya(5))),
        //     ])),
        //     Box::new(Expr::SExpr(vec![
        //         Box::new(Expr::Sym(Symbol::NumOp(NumOp::Divide))),
        //         Box::new(Expr::Num(Sankhya(-10))),
        //         Box::new(Expr::Num(Sankhya(2))),
        //     ])),
        //     Box::new(Expr::SExpr(vec![
        //         Box::new(Expr::Sym(Symbol::NumOp(NumOp::Subtract))),
        //         Box::new(Expr::Num(Sankhya(-1))),
        //         Box::new(Expr::Num(Sankhya(5))),
        //     ])),
        // ]);
        // let env = Env::new(HashMap::new(), None);

        // assert_eq!(eval(&env, &input), Ok(Expr::Num(Sankhya(6))));
    }
}

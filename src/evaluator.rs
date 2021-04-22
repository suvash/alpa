use crate::core;
use crate::environment::Env;
use crate::types::{Error, Expr, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

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
                Expr::Sym(Symbol::NumOp(NumOp::Add)) => core::nums_add(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Subtract)) => core::nums_subtract(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Multiply)) => core::nums_multiply(env, exprs),
                Expr::Sym(Symbol::NumOp(NumOp::Divide)) => core::nums_divide(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::First)) => core::qexpr_first(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Rest)) => core::qexpr_rest(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Len)) => core::qexpr_len(env, exprs),
                Expr::Sym(Symbol::QExprOp(QExprOp::Eval)) => core::qexpr_eval(env, exprs),
                Expr::Sym(Symbol::QExprsOp(QExprsOp::Cons)) => core::qexprs_cons(env, exprs),
                Expr::Sym(Symbol::QExprsOp(QExprsOp::Join)) => core::qexprs_join(env, exprs),
                Expr::Sym(Symbol::SExprOp(SExprOp::Quote)) => core::sexpr_quote(exprs),
                x => Err(Error::InvalidOp(x.clone())),
            },
        },
        Expr::QExpr(_) => Ok(expr.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ntypes::Sankhya;
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

use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::{self, Env};
use crate::types::{Error, Expr, Function, Symbol};

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Sym(sym) => environment::lookup(env, &sym),
        Expr::SExpr(sexpr) => match &**sexpr {
            [] => Ok(expr.clone()),
            [expr] => eval(env, expr),
            [oper, exprs @ ..] => match eval(env, oper) {
                Ok(Expr::Sym(sym)) => environment::lookup(env, &sym),
                Ok(Expr::SExpr(_)) => eval(env, oper),
                Ok(Expr::Fun(Function::Core(_, cf))) => cf(env, exprs),
                Ok(Expr::Fun(Function::Lambda(syms, body, mut hmap))) => {
                    eval_lambda(env, syms, body, &mut hmap, exprs)
                }
                Ok(Expr::QExpr(q)) => {
                    let mut s: Vec<Box<Expr>> = vec![];
                    s.push(Box::new(Expr::SExpr(q)));
                    s.extend_from_slice(&exprs);
                    eval(env, &Expr::SExpr(s))
                }
                _ => Err(Error::InvalidOp(*oper.clone())),
            },
        },
        Expr::QExpr(_) => Ok(expr.clone()),
        Expr::Fun(_) => Ok(expr.clone()),
    }
}

fn eval_lambda(
    parent: &Env,
    formals: Vec<Symbol>,
    body: Box<Expr>,
    hmap: &mut HashMap<Symbol, Expr>,
    args: &[Box<Expr>],
) -> Result<Expr, Error> {
    match args.len() > formals.len() {
        true => Err(Error::TooManyLambdaArguments(formals.len(), args.len())),
        false => {
            let (formals_to_bind, unbound_formals) = &formals.split_at(args.len());

            formals_to_bind.iter().zip(args.iter()).for_each(|z| {
                hmap.insert(z.0.clone(), *z.1.clone());
            });

            match unbound_formals.is_empty() {
                true => {
                    let mut env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                    environment::load_core_fns(&env);
                    let expr: Expr = Expr::SExpr(vec![
                        Box::new(Expr::Sym(Symbol::QExprOp(crate::types::QExprOp::Eval))),
                        body,
                    ]);
                    println!("{:?}", &expr);
                    eval(&mut env, &expr)
                }
                false => Ok(Expr::Fun(Function::Lambda(
                    unbound_formals.to_vec(),
                    body,
                    hmap.clone(),
                ))),
            }
        }
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

use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::{self, Env};
use crate::types::{Error, Expr, Function, Symbol};

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Expr, Error> {
    match expr {
        Expr::Bool(_) => Ok(expr.clone()),
        Expr::Num(_) => Ok(expr.clone()),
        Expr::Str(_) => Ok(expr.clone()),
        Expr::Sym(sym) => environment::lookup(env, &sym),
        Expr::SExpr(sexpr) => match &**sexpr {
            [] => Ok(expr.clone()),
            [expr] => eval(env, expr),
            [oper, exprs @ ..] => match eval(env, oper) {
                Ok(Expr::Sym(sym)) => {
                    let mut n_exprs = vec![];
                    n_exprs.push(Box::new(environment::lookup(env, &sym)?));
                    n_exprs.extend_from_slice(exprs);
                    eval(env, &Expr::SExpr(n_exprs))
                }
                Ok(Expr::Fun(Function::Core(_, cf))) => cf(env, exprs),
                Ok(Expr::Fun(Function::Lambda(syms, body, mut hmap))) => {
                    eval_lambda(env, syms, body, &mut hmap, exprs)
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
    let rest_sym = Symbol::Identifier("ऽ".to_string());
    match &*formals {
        [head_formals @ .., rest, rest_formal] if rest == &rest_sym => {
            println!("{:?}' {:?}, {:?}", head_formals, rest_sym, rest_formal);

            match head_formals.len() <= args.len() {
                true => {
                    let (args_to_bind, rest_args) = &args.split_at(head_formals.len());
                    for z in head_formals.iter().zip(args_to_bind.iter()) {
                        let mut f_env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                        let val = eval(&mut f_env, &*z.1)?;
                        hmap.insert(z.0.clone(), val);
                    }
                    hmap.insert(rest_formal.clone(), Expr::QExpr(rest_args.to_vec()));

                    let mut env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                    let expr: Expr = Expr::SExpr(vec![
                        Box::new(Expr::Sym(Symbol::QExprOp(crate::types::QExprOp::Eval))),
                        body,
                    ]);

                    eval(&mut env, &expr)
                }
                false => {
                    let (formals_to_bind, rest_formals) = &head_formals.split_at(args.len());
                    for z in formals_to_bind.iter().zip(args.iter()) {
                        let mut f_env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                        let val = eval(&mut f_env, &*z.1)?;
                        hmap.insert(z.0.clone(), val);
                    }

                    let mut unbound_formals = rest_formals.to_vec();
                    unbound_formals.push(rest_sym.clone());
                    unbound_formals.push(rest_formal.clone());

                    Ok(Expr::Fun(Function::Lambda(
                        unbound_formals.to_vec(),
                        body,
                        hmap.clone(),
                    )))
                }
            }
        }
        _ => match formals.len() <= args.len() {
            true => {
                let (args_to_bind, rest_args) = &args.split_at(formals.len());
                for z in formals.iter().zip(args_to_bind.iter()) {
                    let mut f_env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                    let val = eval(&mut f_env, &*z.1)?;
                    hmap.insert(z.0.clone(), val);
                }
                hmap.insert(rest_sym.clone(), Expr::QExpr(rest_args.to_vec()));

                let mut env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                let expr: Expr = Expr::SExpr(vec![
                    Box::new(Expr::Sym(Symbol::QExprOp(crate::types::QExprOp::Eval))),
                    body,
                ]);

                eval(&mut env, &expr)
            }
            false => {
                let (formals_to_bind, rest_formals) = &formals.split_at(args.len());
                for z in formals_to_bind.iter().zip(args.iter()) {
                    let mut f_env = environment::new(hmap.clone(), Some(Rc::clone(parent)));
                    let val = eval(&mut f_env, &*z.1)?;
                    hmap.insert(z.0.clone(), val);
                }

                Ok(Expr::Fun(Function::Lambda(
                    rest_formals.to_vec(),
                    body,
                    hmap.clone(),
                )))
            }
        },
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

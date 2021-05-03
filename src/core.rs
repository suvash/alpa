use std::collections::HashMap;

use crate::environment::{self, Env};
use crate::evaluator;
use crate::ntypes::Sankhya;
use crate::types::{Error, Expr, Boolean, Function, NumOp, QExprOp, QExprsOp, SExprOp};

pub type CoreFn = fn(&mut Env, &[Box<Expr>]) -> Result<Expr, Error>;

macro_rules! nums_fn {
    ($fn_name:ident, $op:expr, $x:ident, $y:ident, $x_y_body:block) => {
        pub fn $fn_name(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
            match &exprs[..] {
                [first, rest @ ..] => match evaluator::eval(env, &**first)? {
                    Expr::Num(s) => rest.iter().fold(Ok(Expr::Num(s)), |a, b| {
                        match (a?, evaluator::eval(env, b)?) {
                            (Expr::Num($x), Expr::Num($y)) => $x_y_body,
                            (_, y) => Err(Error::NotANumber(y)),
                        }
                    }),
                    x => Err(Error::NotANumber(x)),
                },
                _ => Err(Error::InvalidNumberOfNumArguments($op, exprs.len())),
            }
        }
    };
}

nums_fn!(nums_add, NumOp::Add, x, y, {
    Ok(Expr::Num(Sankhya(x.0 + y.0)))
});

nums_fn!(nums_subtract, NumOp::Subtract, x, y, {
    Ok(Expr::Num(Sankhya(x.0 - y.0)))
});

nums_fn!(nums_multiply, NumOp::Multiply, x, y, {
    Ok(Expr::Num(Sankhya(x.0 * y.0)))
});

nums_fn!(nums_divide, NumOp::Divide, x, y, {
    match y.0 {
        0 => Err(Error::DivideByZero(x, y)),
        _ => Ok(Expr::Num(Sankhya(x.0 / y.0))),
    }
});

nums_fn!(nums_gt, NumOp::GreaterThan, x, y, {
    Ok(Expr::Bool(Boolean(x.0 > y.0)))
});

nums_fn!(nums_gte, NumOp::GreaterThanOrEqual, x, y, {
    Ok(Expr::Bool(Boolean(x.0 >= y.0)))
});

nums_fn!(nums_lt, NumOp::LessThan, x, y, {
    Ok(Expr::Bool(Boolean(x.0 < y.0)))
});

nums_fn!(nums_lte, NumOp::LessThanOrEqual, x, y, {
    Ok(Expr::Bool(Boolean(x.0 <= y.0)))
});

macro_rules! qexpr_fn {
    ($fn_name:ident, $op:expr, $env:ident, $qexpr:ident, $qexpr_body:block) => {
        pub fn $fn_name($env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
            match &exprs[..] {
                [expr] => match evaluator::eval($env, &expr)? {
                    Expr::QExpr($qexpr) => $qexpr_body,
                    x => Err(Error::NotAQExpr(x.clone())),
                },
                _ => Err(Error::InvalidNumberOfQExprArguments($op, exprs.len())),
            }
        }
    };
}

qexpr_fn!(qexpr_first, QExprOp::First, env, qexpr, {
    match qexpr.split_first() {
        Some((first, _)) => Ok(Expr::QExpr(vec![first.clone()])),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
});

qexpr_fn!(qexpr_rest, QExprOp::Rest, env, qexpr, {
    match qexpr.split_first() {
        Some((_, rest)) => Ok(Expr::QExpr(rest.to_vec())),
        None => Err(Error::EmptyQExpr(Expr::QExpr(qexpr.clone()))),
    }
});

qexpr_fn!(qexpr_len, QExprOp::Len, env, qexpr, {
    Ok(Expr::Num(Sankhya(qexpr.len() as i32)))
});

qexpr_fn!(qexpr_eval, QExprOp::Eval, env, qexpr, {
    evaluator::eval(env, &Expr::SExpr(qexpr.to_vec()))
});

pub fn qexprs_cons(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [pref_expr, expr] => match evaluator::eval(env, &**expr)? {
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

pub fn qexprs_join(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfQExprsArguments(
            QExprsOp::Join,
            exprs.len(),
        )),
        _ => exprs.iter().fold(Ok(Expr::QExpr(vec![])), |a, b| {
            match (a?, evaluator::eval(env, b)?) {
                (Expr::QExpr(mut x), Expr::QExpr(y)) => {
                    x.extend(y);
                    Ok(Expr::QExpr(x))
                }
                (_, y) => Err(Error::NotAQExpr(y)),
            }
        }),
    }
}

macro_rules! qexprs_assign_fn {
    ($fn_name:ident, $env:ident, $sym:ident, $expr:ident, $sym_expr_stmt:stmt) => {
        pub fn $fn_name($env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
            match &exprs[..] {
                [first, rest @ ..] => match evaluator::eval($env, &**first)? {
                    Expr::QExpr(qexpr) => match qexpr.len() == rest.len() {
                        true => {
                            let (_syms, non_syms): (Vec<_>, Vec<_>) = qexpr
                                .iter()
                                .zip(rest.iter())
                                .map(|z| match (&**z.0, evaluator::eval($env, &**z.1)?) {
                                    (Expr::Sym($sym), $expr) => {
					$sym_expr_stmt
					Ok(Expr::SExpr(vec![]))
				    }
                                    (x, _) => Err(Error::NotASymbol(x.clone())),
                                })
				.partition(Result::is_ok);

			    match non_syms.first() {
				Some(first) => first.clone(),
				None => Ok(Expr::SExpr(vec![]))
			    }
                        }
                        false => Err(Error::UnEqualDefList(*first.clone(), rest.to_vec())),
                    },
                    _ => Err(Error::NotAQExpr(*first.clone())),
                },

                _ => Err(Error::InvalidNumberOfQExprsArguments(
                    QExprsOp::Def,
                    exprs.len(),
                )),
            }
        }
    };
}

qexprs_assign_fn!(qexprs_def, env, sym, expr, {
    environment::bind_global_symbol(env, sym, &expr);
});

qexprs_assign_fn!(qexprs_put, env, sym, expr, {
    environment::bind_local_symbol(env, sym, &expr);
});

pub fn qexprs_lambda(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [q_syms, q_body] => match (
            evaluator::eval(env, &**q_syms)?,
            evaluator::eval(env, &**q_body)?,
        ) {
            (Expr::QExpr(qexpr), Expr::QExpr(body)) => {
                let mut sym_exprs = vec![];
                let mut non_sym_exprs = vec![];

                qexpr.iter().for_each(|q| match &**q {
                    Expr::Sym(sym) => sym_exprs.push(sym.clone()),
                    x => non_sym_exprs.push(x.clone()),
                });

                match non_sym_exprs.first() {
                    Some(first) => Err(Error::NotASymbol(first.clone())),
                    None => Ok(Expr::Fun(Function::Lambda(
                        sym_exprs,
                        Box::new(Expr::QExpr(body.clone())),
                        HashMap::new(),
                    ))),
                }
            }
            (Expr::QExpr(_), expr) => Err(Error::NotAQExpr(expr.clone())),
            (expr, _) => Err(Error::NotAQExpr(expr.clone())),
        },
        _ => Err(Error::InvalidNumberOfQExprsArguments(
            QExprsOp::Lambda,
            exprs.len(),
        )),
    }
}

pub fn sexpr_quote(_env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [] => Err(Error::InvalidNumberOfSExprArguments(
            SExprOp::Quote,
            exprs.len(),
        )),
        _ => Ok(Expr::QExpr(exprs.to_vec())),
    }
}

pub fn sexpr_printenv(env: &mut Env, _exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    println!("{:#?}", env);
    Ok(Expr::QExpr(vec![]))
}

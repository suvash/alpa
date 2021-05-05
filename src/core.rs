use std::collections::HashMap;
use std::fs;

use crate::environment::{self, Env};
use crate::evaluator;
use crate::ntypes::Sankhya;
use crate::parser;
use crate::types::{
    Boolean, Error, Expr, ExprsOp, Function, NumOp, QExprOp, QExprsOp, SExprOp, Symbol,
};

pub type CoreFn = fn(&mut Env, &[Box<Expr>]) -> Result<Expr, Error>;

pub fn exprs_if(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [cond_expr, if_expr, else_expr] => match evaluator::eval(env, &**cond_expr)? {
            Expr::Bool(Boolean(b)) => match (&**if_expr, &**else_expr) {
                (Expr::QExpr(q1), Expr::QExpr(q2)) => match b {
                    true => evaluator::eval(env, &Expr::SExpr(q1.to_vec())),
                    false => evaluator::eval(env, &Expr::SExpr(q2.to_vec())),
                },
                (Expr::QExpr(_), x) => Err(Error::NotAQExpr(x.clone())),
                (x, _) => Err(Error::NotAQExpr(x.clone())),
            },
            x => Err(Error::NotABoolean(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfExprsArguments(
            ExprsOp::If,
            exprs.len(),
        )),
    }
}

pub fn exprs_equal(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr1, expr2] => exprs_compare(env, expr1, expr2),
        _ => Err(Error::InvalidNumberOfExprsArguments(
            ExprsOp::Equal,
            exprs.len(),
        )),
    }
}

fn exprs_compare(env: &mut Env, expr1: &Expr, expr2: &Expr) -> Result<Expr, Error> {
    match (expr1, expr2) {
        (Expr::Bool(b1), Expr::Bool(b2)) => Ok(Expr::Bool(Boolean(b1 == b2))),
        (Expr::Num(n1), Expr::Num(n2)) => Ok(Expr::Bool(Boolean(n1 == n2))),
        (Expr::Str(s1), Expr::Str(s2)) => Ok(Expr::Bool(Boolean(s1 == s2))),
        (Expr::Sym(_), Expr::Sym(_)) => Ok(Expr::Bool(Boolean(
            evaluator::eval(env, expr1)? == evaluator::eval(env, expr2)?,
        ))),
        (Expr::QExpr(q1), Expr::QExpr(q2)) => Ok(Expr::Bool(Boolean(q1 == q2))),
        (Expr::Fun(f1), Expr::Fun(f2)) => Ok(Expr::Bool(Boolean(f1 == f2))),
        (Expr::SExpr(_), _) => {
            let es1 = evaluator::eval(env, expr1)?;
            exprs_compare(env, &es1, expr2)
        }
        (_, Expr::SExpr(_)) => {
            let es2 = evaluator::eval(env, expr2)?;
            exprs_compare(env, expr1, &es2)
        }
        (Expr::Sym(_), _) => {
            let es1 = evaluator::eval(env, expr1)?;
            exprs_compare(env, &es1, expr2)
        }
        (_, Expr::Sym(_)) => {
            let es2 = evaluator::eval(env, expr2)?;
            exprs_compare(env, expr1, &es2)
        }

        _ => Ok(Expr::Bool(Boolean(false))),
    }
}

pub fn exprs_not_equal(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match exprs_equal(env, exprs)? {
        Expr::Bool(Boolean(b)) => Ok(Expr::Bool(Boolean(!b))),
        _ => Ok(Expr::Bool(Boolean(true))),
    }
}

pub fn exprs_import(env: &mut Env, exprs: &[Box<Expr>]) -> Result<Expr, Error> {
    match &exprs[..] {
        [expr] => match &**expr {
            Expr::Sym(Symbol::Identifier(target)) => {
                let extension = "अ";
                let filename = format!("{}.{}", target, extension);
                match fs::read_to_string(filename) {
                    Ok(contents) => {
                        println!("{:?}", &contents);
                        match parser::parse(&contents) {
                            Err(pe) => {
                                eprintln!("Could not parse :\n{:?}", pe);
                                Err(Error::ParseError(String::from(&contents)))
                            }
                            Ok(Expr::SExpr(pexprs)) => {
                                println!("Parsed : {:?}", &pexprs);

                                for pexpr in pexprs.iter() {
                                    let val = evaluator::eval(env, pexpr)?;
                                    println!("{}", &val);
                                }

                                Ok(Expr::QExpr(vec![]))
                            }
                            Ok(_) => unreachable!(),
                        }
                    }
                    Err(_) => Err(Error::ImportError(*expr.clone())),
                }
            }
            x => Err(Error::NotAnIdentifier(x.clone())),
        },
        _ => Err(Error::InvalidNumberOfExprsArguments(
            ExprsOp::Import,
            exprs.len(),
        )),
    }
}

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

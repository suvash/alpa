use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::core::{self, CoreFn};
use crate::types::{Error, Expr, ExprsOp, Function, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

#[derive(Debug)]
pub struct EnvCtx {
    data: RefCell<HashMap<Symbol, Expr>>,
    pub parent: Option<Env>,
}

pub type Env = Rc<EnvCtx>;

pub fn new(hmap: HashMap<Symbol, Expr>, parent: Option<Env>) -> Env {
    Rc::new(EnvCtx {
        data: RefCell::new(hmap),
        parent,
    })
}

pub fn lookup(env: &Env, symbol: &Symbol) -> Result<Expr, Error> {
    match env.data.borrow().get(symbol) {
        Some(expr) => Ok(expr.clone()),
        None => match &env.parent {
            None => Err(Error::UnboundSymbol(symbol.clone())),
            Some(parent) => lookup(&parent, symbol),
        },
    }
}

fn insert(env: &Env, symbol: &Symbol, expr: &Expr) {
    env.data.borrow_mut().insert(symbol.clone(), expr.clone());
}

pub fn bind_local_symbol(env: &Env, symbol: &Symbol, expr: &Expr) {
    insert(env, symbol, expr);
}

fn root(env: &Env) -> &Env {
    match &env.parent {
        None => env,
        Some(parenv) => root(&parenv),
    }
}

pub fn bind_global_symbol(env: &Env, symbol: &Symbol, expr: &Expr) {
    root(env)
        .data
        .borrow_mut()
        .insert(symbol.clone(), expr.clone());
}

fn bind_global_core_fn(env: &Env, symbol: Symbol, func: CoreFn) {
    root(env)
        .data
        .borrow_mut()
        .insert(symbol.clone(), Expr::Fun(Function::Core(symbol, func)));
}

pub fn load_core_fns(env: &Env) {
    bind_global_core_fn(env, Symbol::ExprsOp(ExprsOp::If), core::exprs_if);
    bind_global_core_fn(env, Symbol::ExprsOp(ExprsOp::Equal), core::exprs_equal);
    bind_global_core_fn(
        env,
        Symbol::ExprsOp(ExprsOp::NotEqual),
        core::exprs_not_equal,
    );
    bind_global_core_fn(env, Symbol::ExprsOp(ExprsOp::Import), core::exprs_import);
    bind_global_core_fn(env, Symbol::ExprsOp(ExprsOp::Print), core::exprs_print);
    bind_global_core_fn(env, Symbol::ExprsOp(ExprsOp::Error), core::exprs_error);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::Add), core::nums_add);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::Subtract), core::nums_subtract);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::Multiply), core::nums_multiply);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::Divide), core::nums_divide);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::Multiply), core::nums_multiply);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::GreaterThan), core::nums_gt);
    bind_global_core_fn(
        env,
        Symbol::NumOp(NumOp::GreaterThanOrEqual),
        core::nums_gte,
    );
    bind_global_core_fn(env, Symbol::NumOp(NumOp::LessThan), core::nums_lt);
    bind_global_core_fn(env, Symbol::NumOp(NumOp::LessThanOrEqual), core::nums_lte);
    bind_global_core_fn(env, Symbol::QExprOp(QExprOp::First), core::qexpr_first);
    bind_global_core_fn(env, Symbol::QExprOp(QExprOp::Rest), core::qexpr_rest);
    bind_global_core_fn(env, Symbol::QExprOp(QExprOp::Len), core::qexpr_len);
    bind_global_core_fn(env, Symbol::QExprOp(QExprOp::Eval), core::qexpr_eval);
    bind_global_core_fn(env, Symbol::QExprsOp(QExprsOp::Cons), core::qexprs_cons);
    bind_global_core_fn(env, Symbol::QExprsOp(QExprsOp::Join), core::qexprs_join);
    bind_global_core_fn(env, Symbol::QExprsOp(QExprsOp::Def), core::qexprs_def);
    bind_global_core_fn(env, Symbol::QExprsOp(QExprsOp::Put), core::qexprs_put);
    bind_global_core_fn(env, Symbol::QExprsOp(QExprsOp::Lambda), core::qexprs_lambda);
    bind_global_core_fn(env, Symbol::SExprOp(SExprOp::Quote), core::sexpr_quote);
    bind_global_core_fn(
        env,
        Symbol::SExprOp(SExprOp::PrintEnv),
        core::sexpr_printenv,
    );
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::ntypes::Sankhya;

    #[test]
    fn test_env() {
        // let mut store = HashMap::new();
        // let dus = Symbol::Identifier("दस".to_string());
        // let ek = Symbol::Identifier("एक".to_string());
        // store.insert(dus.clone(), Expr::Num(Sankhya(10)));

        // let env = Env::new(store, None);
        // assert_eq!(None, env.get(&ek));
        // assert_eq!(Some(&Expr::Num(Sankhya(10))), env.get(&dus));
    }
}

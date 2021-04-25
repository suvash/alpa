use std::collections::HashMap;

use crate::core::{self, CoreFn};
use crate::types::{Error, Expr, Function, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

#[derive(Debug)]
pub struct Env<'a> {
    local: HashMap<Symbol, Expr>,
    pub parent: Option<&'a Env<'a>>,
}

impl<'b> Env<'b> {
    pub fn new(local: HashMap<Symbol, Expr>, parent: Option<&'b Env<'b>>) -> Self {
        Env { local, parent }
    }

    pub fn lookup(&self, symbol: &Symbol) -> Result<Expr, Error> {
        match self.local.get(symbol) {
            Some(expr) => Ok(expr.clone()),
            None => match self.parent {
                None => Err(Error::UnboundSymbol(symbol.clone())),
                Some(parent) => parent.lookup(symbol),
            },
        }
    }

    fn insert_or_update(&mut self, symbol: &Symbol, expr: Expr) {
        let sym_entry = self.local.entry(symbol.clone()).or_insert(expr.clone());
        *sym_entry = expr;
    }

    pub fn bind_local_symbol(&mut self, symbol: &Symbol, expr: Expr) {
        self.insert_or_update(symbol, expr)
    }

    fn bind_core_fn(&mut self, symbol: &Symbol, func: CoreFn) {
        self.insert_or_update(symbol, Expr::Fun(Function::Core(symbol.clone(), func)))
    }

    pub fn load_core_fns(&mut self) {
        self.bind_core_fn(&Symbol::NumOp(NumOp::Add), core::nums_add);
        self.bind_core_fn(&Symbol::NumOp(NumOp::Subtract), core::nums_subtract);
        self.bind_core_fn(&Symbol::NumOp(NumOp::Multiply), core::nums_multiply);
        self.bind_core_fn(&Symbol::NumOp(NumOp::Divide), core::nums_divide);
        self.bind_core_fn(&Symbol::NumOp(NumOp::Multiply), core::nums_multiply);
        self.bind_core_fn(&Symbol::QExprOp(QExprOp::First), core::qexpr_first);
        self.bind_core_fn(&Symbol::QExprOp(QExprOp::Rest), core::qexpr_rest);
        self.bind_core_fn(&Symbol::QExprOp(QExprOp::Len), core::qexpr_len);
        self.bind_core_fn(&Symbol::QExprOp(QExprOp::Eval), core::qexpr_eval);
        self.bind_core_fn(&Symbol::QExprsOp(QExprsOp::Cons), core::qexprs_cons);
        self.bind_core_fn(&Symbol::QExprsOp(QExprsOp::Join), core::qexprs_join);
        self.bind_core_fn(&Symbol::QExprsOp(QExprsOp::Def), core::qexprs_def);
        self.bind_core_fn(&Symbol::SExprOp(SExprOp::Quote), core::sexpr_quote);
        self.bind_core_fn(&Symbol::SExprOp(SExprOp::PrintEnv), core::sexpr_printenv);
    }
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

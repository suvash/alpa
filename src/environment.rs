use std::collections::HashMap;

use crate::types::{Function, Symbol};

#[derive(Debug)]
pub struct Env<'a> {
    store: HashMap<Symbol, Function>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'b> Env<'b> {
    pub fn new(store: HashMap<Symbol, Function>, outer: Option<&'b Env<'b>>) -> Self {
        Env { store, outer }
    }

    pub fn get(&self, symbol: &Symbol) -> Option<&Function> {
        self.store.get(symbol)
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

use rustyline::error::ReadlineError;
use rustyline::Editor;

use alpa::environment::Env;
use alpa::evaluator;
use alpa::parser;
use std::collections::HashMap;

use alpa::core::{self};
use alpa::types::{Expr, Function, NumOp, QExprOp, QExprsOp, SExprOp, Symbol};

fn main() {
    print_banner();
    repl();
}

fn print_banner() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{} version {}", name, version);
    println!("Use Ctrl-C, or Ctrl-D to exit prompt");
    println!();
}

fn repl_env() -> Env<'static> {
    let mut store: HashMap<Symbol, Expr> = HashMap::new();

    store.insert(
        Symbol::NumOp(NumOp::Add),
        Expr::Fun(Function::Core(Symbol::NumOp(NumOp::Add), core::nums_add)),
    );
    store.insert(
        Symbol::NumOp(NumOp::Subtract),
        Expr::Fun(Function::Core(
            Symbol::NumOp(NumOp::Subtract),
            core::nums_subtract,
        )),
    );
    store.insert(
        Symbol::NumOp(NumOp::Multiply),
        Expr::Fun(Function::Core(
            Symbol::NumOp(NumOp::Multiply),
            core::nums_multiply,
        )),
    );
    store.insert(
        Symbol::NumOp(NumOp::Divide),
        Expr::Fun(Function::Core(
            Symbol::NumOp(NumOp::Divide),
            core::nums_divide,
        )),
    );
    store.insert(
        Symbol::NumOp(NumOp::Multiply),
        Expr::Fun(Function::Core(
            Symbol::NumOp(NumOp::Multiply),
            core::nums_multiply,
        )),
    );
    store.insert(
        Symbol::QExprOp(QExprOp::First),
        Expr::Fun(Function::Core(
            Symbol::QExprOp(QExprOp::First),
            core::qexpr_first,
        )),
    );
    store.insert(
        Symbol::QExprOp(QExprOp::Rest),
        Expr::Fun(Function::Core(
            Symbol::QExprOp(QExprOp::Rest),
            core::qexpr_rest,
        )),
    );
    store.insert(
        Symbol::QExprOp(QExprOp::Len),
        Expr::Fun(Function::Core(
            Symbol::QExprOp(QExprOp::Len),
            core::qexpr_len,
        )),
    );
    store.insert(
        Symbol::QExprOp(QExprOp::Eval),
        Expr::Fun(Function::Core(
            Symbol::QExprOp(QExprOp::Eval),
            core::qexpr_eval,
        )),
    );
    store.insert(
        Symbol::QExprsOp(QExprsOp::Cons),
        Expr::Fun(Function::Core(
            Symbol::QExprsOp(QExprsOp::Cons),
            core::qexprs_cons,
        )),
    );
    store.insert(
        Symbol::QExprsOp(QExprsOp::Join),
        Expr::Fun(Function::Core(
            Symbol::QExprsOp(QExprsOp::Join),
            core::qexprs_join,
        )),
    );
    store.insert(
        Symbol::SExprOp(SExprOp::Quote),
        Expr::Fun(Function::Core(
            Symbol::SExprOp(SExprOp::Quote),
            core::sexpr_quote,
        )),
    );

    Env::new(store, None)
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let history_filename = "history.txt";
    if rl.load_history(&history_filename).is_err() {
        eprintln!("Could not find previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let env = repl_env();
                read_eval_print(&env, &line);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&history_filename).unwrap();
}

fn read_eval_print(env: &Env, line: &str) -> () {
    match parser::parse(line) {
        Err(e) => {
            eprintln!("Could not parse");
            eprintln!("{:?}", e);
        }
        Ok(expr) => {
            println!("Parsed : {:?}", &expr);

            match evaluator::eval(env, &expr) {
                Err(e) => {
                    eprintln!("Could not eval");
                    eprintln!("Eval Error : {:?}", e);
                }

                Ok(expr) => {
                    println!("Evaled : {:?}", &expr);
                    println!("{}", &expr);
                }
            }
        }
    }
}

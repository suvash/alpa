use std::collections::HashMap;
use std::env;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use alpa::core;
use alpa::environment::{self, Env};
use alpa::evaluator;
use alpa::parser;
use alpa::types::{Expr, Symbol};

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => {
            print_banner();
            repl();
        }
        [_, f] => {
            let mut env = env_with_core_fns();
            let import_f: &str = f.split('.').collect::<Vec<&str>>().first().unwrap();
            let import = vec![Box::new(Expr::Sym(Symbol::Identifier(String::from(
                import_f,
            ))))];
            core::exprs_import(&mut env, &import);
        }
        _ => {
            eprintln!("Invalid number of arguments.");
        }
    }
}

fn print_banner() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{} version {}", name, version);
    println!("Use Ctrl-C, or Ctrl-D to exit prompt");
    println!();
}

fn env_with_core_fns() -> Env {
    let env = environment::new(HashMap::new(), None);
    environment::load_core_fns(&env);

    env
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let history_filename = "history.txt";
    if rl.load_history(&history_filename).is_err() {
        eprintln!("Could not find previous history.");
    }
    let mut env = env_with_core_fns();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                read_eval_print(&mut env, &line);
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

fn read_eval_print(env: &mut Env, line: &str) -> () {
    match parser::parse(line) {
        Err(e) => {
            eprintln!("Could not parse :\n{:?}", e);
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

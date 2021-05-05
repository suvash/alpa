use rustyline::error::ReadlineError;
use rustyline::Editor;

use alpa::environment::{self, Env};
use alpa::evaluator;
use alpa::parser;
use std::collections::HashMap;

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

fn repl_env() -> Env {
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
    let mut env = repl_env();
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

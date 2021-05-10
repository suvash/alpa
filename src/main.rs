use std::collections::HashMap;
use std::env;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use alpa::core;
use alpa::environment::{self, Env};
use alpa::types::{Expr, Symbol};

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => {
            print_banner();
            repl();
        }
        [_, arg1] => {
            let mut env = env_with_stdlib_and_core_fns();
            eval_import(&mut env, arg1);
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

fn env_with_stdlib_and_core_fns() -> Env {
    let stdlib_content = include_str!("प्रस्तावना.अ");
    let mut env = environment::new(HashMap::new(), None);

    environment::load_core_fns(&env);
    read_eval_print(&mut env, &stdlib_content);

    env
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let history_filename = "history.txt";
    if rl.load_history(&history_filename).is_err() {
        eprintln!("Could not find previous history.");
    }
    let mut env = env_with_stdlib_and_core_fns();
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

fn eval_import(env: &mut Env, target: &str) -> () {
    match target.split('.').collect::<Vec<&str>>()[..] {
        [module, "अ"] => {
            let m = vec![Box::new(Expr::Sym(Symbol::Identifier(String::from(
                module,
            ))))];

            match core::exprs_import(env, &m) {
                Err(e) => eprintln!("Error : {:?}", e),
                Ok(expr) => {
                    println!("{:?}", &expr);
                    println!("{}", &expr);
                }
            }
        }
        _ => eprintln!("अमान्य फाइल (हुनुपर्ने <फाइलनाम>.अ)"),
    }
}

fn read_eval_print(env: &mut Env, line: &str) -> () {
    match core::parse_and_eval_str(env, line) {
        Err(e) => eprintln!("Error : {:?}", e),
        Ok(expr) => {
            println!("{:?}", &expr);
            println!("{}", &expr);
        }
    }
}

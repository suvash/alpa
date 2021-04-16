use rustyline::error::ReadlineError;
use rustyline::Editor;

use alpa::evaluator;
use alpa::parser;
use alpa::types::Expr;

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
                // let evaled = read(&line).and_then(evaluator);
                // printer(evaled);
                let parsed = read(&line).unwrap();
                let evaled = evaluator(&parsed);
                println!("{:?}", &parsed);
                println!("{:?}", &evaled);
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

fn read(line: &str) -> Option<Expr> {
    parser::parse(line)
}

fn evaluator(expr: &Expr) -> Expr {
    evaluator::eval_expr(expr)
}

// fn printer(val: Option<Sankhya>) -> () {
//     match val {
//         Some(val) => println!("{}", val),
//         None => eprintln!("Nothing to print"),
//     }
// }

use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                let evaled = evaluator(&line);
                printer(evaled);
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

fn evaluator(line: &str) -> &str {
    line
}

fn printer(line: &str) -> () {
    println!("{}", line);
}

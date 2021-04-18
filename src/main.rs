#![allow(unused)]

use chrono::DateTime;
use chrono::Utc;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{
    env,
    io::{Read, Write},
};
use std::{fs, str::Chars};

mod interpreter;
mod lexer;
mod number;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(filename) = args.get(1) {
        run_file(filename);
    } else {
        run_repl();
    }
}

fn run_file(filename: &String) {
    let source = fs::read_to_string(filename).unwrap();
    let tokens = lexer::lex(&source).expect("Failed to lex");
    let ast = parser::parse(tokens).expect("Failed to parse");
    interpreter::visit(ast);
}

fn run_repl() {
    let mut rl = Editor::<()>::new();
    rl.load_history("history.txt");

    let utc: DateTime<Utc> = Utc::now();

    println!(
        "Cringelang😳 0.1.0 [{} on {} {}]
Ctrl-C to exit",
        utc.format("%b %d %Y, %H:%M:%S").to_string(),
        env::consts::OS,
        env::consts::ARCH
    );

    loop {
        let mut input = rl.readline(">>> ");
        match input {
            Ok(s) => {
                if s.is_empty() {
                    continue;
                }
                rl.add_history_entry(s.as_str());
                let tokens = match lexer::lex(&s) {
                    Ok(a) => a,
                    Err(msg) => {
                        eprintln!("{}", msg);
                        continue;
                    }
                };
                // println!("{:?}", tokens);
                let ast = match parser::parse(tokens) {
                    Ok(ast) => ast,
                    Err(msg) => {
                        eprintln!("{}", msg);
                        continue;
                    }
                };
                // println!("{:?}", ast);
                let res = interpreter::visit(ast);
                match res {
                    Ok(res) => println!("{}", res),
                    Err(err) => eprintln!("Runtime error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt");
}

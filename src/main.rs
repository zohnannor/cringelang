#![allow(unused)]

mod parse;
mod stdlib;

use chrono::{DateTime, Utc};
use parse::{interpreter, lexer, parser, Context};
use rustyline::{error::ReadlineError, Editor};
use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
    str::Chars,
};
use stdlib::number::Number;

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

    let mut global_sym_table = Context::new();
    interpreter::visit(ast, &mut global_sym_table);
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

    let mut global_sym_table = Context::new();

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
                let res = interpreter::visit(ast, &mut global_sym_table);
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

mod cli;
mod parse;
mod stdlib;

use cli::run_repl;
use parse::{interpreter, lexer, parser, Context};
use std::{env, fs};

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
    interpreter::visit(ast, &mut global_sym_table).expect("Fatal error");
}

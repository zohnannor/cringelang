mod cli;
mod parse;
mod stdlib;

use cli::run_repl;
use parse::{interpreter, lexer, parser, Context};
use std::{env, fs};

fn main() {
    match env::args().nth(1) {
        Some(filename) => run_file(&filename),
        None => run_repl(),
    }
}

fn run_file(filename: &str) {
    let source = fs::read_to_string(filename).unwrap();
    let tokens = lexer::lex(&source).expect("Failed to lex");
    let ast = parser::parse(tokens).expect("Failed to parse");

    let mut global_sym_table = Context::new();
    interpreter::visit(ast, &mut global_sym_table).expect("Fatal error");
}

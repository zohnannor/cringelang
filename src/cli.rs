use chrono::{DateTime, Utc};
use colored::*;
use rustyline::{
    completion::Completer,
    error::ReadlineError,
    highlight::Highlighter,
    hint::{Hinter, HistoryHinter},
    validate::Validator,
    Editor, Helper,
};
use std::{
    borrow::Cow::{self, Owned},
    env,
};

use crate::{
    parse::{interpreter, lexer, parser},
    stdlib::object::Object,
    Context,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct MyHelper {
    hinter: HistoryHinter,
}

impl MyHelper {
    fn new() -> Self {
        Self {
            hinter: HistoryHinter {},
        }
    }

    fn is_contains_number(&self, line: &str) -> bool {
        // TODO: floats, inf and NaN
        line.chars().any(|c| c.is_ascii_digit())
    }

    fn is_contains_bools(&self, line: &str) -> bool {
        line.contains("true") || line.contains("false")
    }
}

impl Helper for MyHelper {}
impl Validator for MyHelper {}
impl Completer for MyHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let _ = (line, pos, ctx);
        if let Some(s) = self.hinter.hint(line, pos, ctx) {
            return Ok((pos, vec![s]));
        };
        Ok((0, Vec::with_capacity(0)))
    }

    fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected)
    }
}

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        let mut copy = line.to_owned();
        if self.is_contains_number(line) {
            copy = highlight_numbers(&copy);
        }
        if self.is_contains_bools(line) {
            copy = highlight_bools(&copy);
        }
        Owned(copy)
    }
}

fn highlight_numbers(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_string().yellow().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn highlight_bools(s: &str) -> String {
    s.to_string()
        .replace("true", &"true".purple().to_string())
        .replace("false", &"false".purple().to_string())
}

fn highlight_items(res: Object) -> String {
    highlight_bools(&highlight_numbers(&format!("{}", res)))
}

pub fn run_repl() {
    let h = MyHelper::new();
    let mut rl = Editor::<MyHelper>::new();
    rl.set_helper(Some(h));
    rl.load_history("history.txt").ok();

    let utc: DateTime<Utc> = Utc::now();

    println!(
        "Cringelang😳 {} [{} on {} {}]\nCtrl-C to exit",
        VERSION,
        utc.format("%b %d %Y, %H:%M:%S").to_string(),
        env::consts::OS,
        env::consts::ARCH
    );

    let mut global_sym_table = Context::new();

    loop {
        let input = rl.readline(">>> ");
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

                #[cfg(feature = "debug")]
                println!("{:?}", tokens);

                let ast = match parser::parse(tokens) {
                    Ok(ast) => ast,
                    Err(msg) => {
                        eprintln!("{}", msg);
                        continue;
                    }
                };

                #[cfg(feature = "debug")]
                println!("{:?}", ast);

                let res = interpreter::visit(ast, &mut global_sym_table);
                match res {
                    Ok(res) => println!("{}", highlight_items(res)),
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
    rl.save_history("history.txt").unwrap();
}

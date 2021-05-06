use std::{iter::Peekable, str::Chars};

use super::tokens::{Operator, Parenthesis, Token};
use crate::stdlib::number::Number;

use Operator::*;

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut source: Peekable<Chars> = source.chars().peekable();

    while let Some(ch) = source.peek() {
        match ch {
            '0'..='9' | '.' => {
                tokens.push(Token::Number(make_number(&mut source)?));
            }
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                tokens.push(Token::Ident(make_name(&mut source)));
            }
            '=' | '+' | '-' | '*' | '/' | '%' | '!' | '>' | '<' | ':' | ';' | '(' | ')' | '['
            | ']' | '{' | '}' => {
                tokens.push(Token::Operator(make_operator(&mut source)));
            }
            ' ' | '\n' | '\r' | '\t' => {
                source.next();
            }
            _ => {
                return Err(format!("Unexpected token: `{}`", source.next().unwrap()));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn make_number(source: &mut Peekable<Chars>) -> Result<Number, String> {
    let mut dot_count = if source.peek().unwrap() == &'.' { 1 } else { 0 };
    let mut number = source.next().unwrap().to_string();
    while let Some(&ch) = source.peek() {
        match ch {
            '0'..='9' => {
                number.push(ch);
                source.next();
            }
            '.' => {
                number.push(ch);
                dot_count += 1;
                if dot_count > 1 {
                    return Err("Syntax Error".to_string());
                };
                source.next();
            }
            _ => break,
        };
    }

    Ok(if dot_count == 1 {
        Number::Float(number.parse::<f64>().map_err(|err| err.to_string())?)
    } else {
        Number::Int(number.parse::<i128>().map_err(|err| err.to_string())?)
    })
}

fn make_name(source: &mut Peekable<Chars>) -> String {
    let mut name = source.next().unwrap().to_string();
    while let Some(&ch) = source.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$' => {
                name.push(ch);
                source.next();
            }
            _ => break,
        }
    }
    name
}

fn make_operator(source: &mut Peekable<Chars>) -> Operator {
    let ch = source.next().unwrap();
    match ch {
        '+' => Plus,
        '-' => Minus,
        '/' => Slash,
        '%' => Percent,
        ':' => Colon,
        ';' => Semicolon,
        '|' => Pipe,
        '&' => Ampersand,
        '^' => Caret,
        '.' => Dot,
        '>' => make_3char_long_operator(source, '=', '>', (Greater, GreaterEquals, GreaterGreater)),
        '<' => make_3char_long_operator(source, '=', '<', (Less, LessEquals, LessLess)),
        '=' => make_2char_long_operator(source, '=', (Equals, EqualsEquals)),
        '*' => make_2char_long_operator(source, '*', (Star, StarStar)),
        '!' => make_2char_long_operator(source, '=', (Exclamation, ExclamationEquals)),
        '(' => Parenthesis(Parenthesis::LParen),
        ')' => Parenthesis(Parenthesis::RParen),
        '[' => Parenthesis(Parenthesis::LBracket),
        ']' => Parenthesis(Parenthesis::RBracket),
        '{' => Parenthesis(Parenthesis::LCurly),
        '}' => Parenthesis(Parenthesis::RCurly),
        _ => unreachable!(),
    }
}

fn make_2char_long_operator(
    source: &mut Peekable<Chars>,
    c: char,
    operators: (Operator, Operator),
) -> Operator {
    match source.peek() {
        Some(&ch) => match ch {
            ch if ch == c => {
                source.next();
                operators.1
            }
            _ => operators.0,
        },
        _ => operators.0,
    }
}

fn make_3char_long_operator(
    source: &mut Peekable<Chars>,
    c1: char,
    c2: char,
    operators: (Operator, Operator, Operator),
) -> Operator {
    match source.peek() {
        Some(&ch) => match ch {
            ch if ch == c1 => {
                source.next();
                operators.1
            }
            ch if ch == c2 => {
                source.next();
                operators.2
            }
            _ => operators.0,
        },
        _ => operators.0,
    }
}

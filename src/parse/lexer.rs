use std::{fmt::Display, iter::Peekable, str::Chars};

use crate::stdlib::number::Number;

#[derive(Debug, Clone, Copy)]
pub enum Parenthesis {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Equals,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Percent,
    Colon,
    Semicolon,
    Parenthesis(Parenthesis),
}

#[derive(Debug)]
pub enum Token {
    Number(Number),
    Ident(String),
    Operator(Operator),
    Eof,
}

pub fn lex(source: &String) -> Result<Vec<Token>, String> {
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
            '=' | '+' | '-' | '*' | '/' | '%' | ':' | ';' | '(' | ')' | '[' | ']' | '{' | '}' => {
                tokens.push(Token::Operator(make_operator(&mut source)));
            }
            ' ' | '\n' | '\r' | '\t' => {
                source.next();
            }
            _ => {
                return Err(format!("Unexpected token: `{}`", source.next().unwrap()));
                source.next();
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
                    return Err(format!("Syntax Error"));
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
        '=' => Operator::Equals,
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '*' => match source.peek() {
            Some(ch) => match ch {
                '*' => {
                    source.next();
                    Operator::StarStar
                }
                _ => Operator::Star,
            },
            _ => Operator::Star,
        },
        '/' => Operator::Slash,
        '%' => Operator::Percent,
        ':' => Operator::Colon,
        ';' => Operator::Semicolon,
        '(' => Operator::Parenthesis(Parenthesis::LParen),
        ')' => Operator::Parenthesis(Parenthesis::RParen),
        '[' => Operator::Parenthesis(Parenthesis::LBracket),
        ']' => Operator::Parenthesis(Parenthesis::RBracket),
        '{' => Operator::Parenthesis(Parenthesis::LCurly),
        '}' => Operator::Parenthesis(Parenthesis::RCurly),
        _ => unreachable!(),
    }
}

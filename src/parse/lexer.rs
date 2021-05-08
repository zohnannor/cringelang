use std::{iter::Peekable, str::Chars};

use super::tokens::{
    Operator::{self, *},
    Parenthesis, Token,
};

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
            | ']' | '{' | '}' | '|' | '^' | '&' => {
                tokens.push(Token::Operator(make_operator(&mut source)));
            }
            ' ' | '\n' | '\r' | '\t' => {
                source.next();
            }
            '"' => {
                tokens.push(Token::String(make_string(&mut source)?));
            }
            '\'' => {
                tokens.push(Token::Char(make_char(&mut source)?));
            }
            _ => {
                return Err(format!("Unexpected token: `{}`", source.next().unwrap()));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn make_string(source: &mut Peekable<Chars>) -> Result<String, String> {
    source.next();
    let mut s = String::new();
    while let Some(&ch) = source.peek() {
        match ch {
            '"' => break,
            '\\' => s.push(make_escape_sequence_char(source)?),
            _ => s.push(source.next().unwrap()),
        };
    }
    match source.peek() {
        Some('"') => {
            source.next();
            Ok(s)
        }
        _ => Err("Syntax Error: Expected closing `\"`".to_string()),
    }
}

fn make_char(source: &mut Peekable<Chars>) -> Result<char, String> {
    source.next();
    let c = match source.peek() {
        Some(&ch) => match ch {
            '\\' => make_escape_sequence_char(source)?,
            '\'' => return Err("Syntax Error: Empty char literal".to_string()),
            _ => source.next().unwrap(),
        },
        None => return Err("Syntax Error: Unexpected EOL".to_string()),
    };
    match source.peek() {
        Some('\'') => {
            source.next();
            Ok(c)
        }
        Some(_) => Err("Syntax Error: Literal must me one character long ".to_string()),
        None => Err("Syntax Error: Expected closing `\'`".to_string()),
    }
}

fn make_escape_sequence_char(source: &mut Peekable<Chars>) -> Result<char, String> {
    source.next();
    let c = match source.peek() {
        Some('n') => '\n',
        Some('r') => '\r',
        Some('t') => '\t',
        Some('0') => '\0',
        Some('"') => '"',
        Some('\\') => '\\',
        Some('\'') => '\'',
        Some(c) => {
            return Err(format!(
                "Syntax Error: Unknown escape sequence character: {:?}",
                *c
            ))
        }
        None => return Err("Syntax Error: Unexpected EOL".to_string()),
    };
    source.next();
    Ok(c)
}

fn make_number(source: &mut Peekable<Chars>) -> Result<f64, String> {
    // let mut dot_count = if source.peek().unwrap() == &'.' { 1 } else { 0 };
    let mut number = source.next().unwrap().to_string();
    while let Some(&ch) = source.peek() {
        match ch {
            '0'..='9' => {
                number.push(ch);
                source.next();
            }
            // '.' => {
            //     number.push(ch);
            //     dot_count += 1;
            //     if dot_count > 1 {
            //         return Err("Syntax Error".to_string());
            //     };
            //     source.next();
            // }
            _ => break,
        };
    }

    // Ok(if dot_count == 1 {
    number.parse::<f64>().map_err(|err| err.to_string())
    // } else {
    //     Number::Int(number.parse::<i128>().map_err(|err| err.to_string())?)
    // })
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

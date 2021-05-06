use std::{iter::Peekable, slice::Iter};

use super::tokens::{Operator, Parenthesis, Token};
use crate::stdlib::{bool::Bool, number::Number};

const KEYWORDS: [&str; 5] = ["let", "inf", "NaN", "true", "false"];

#[derive(Debug)]
pub enum AstNode {
    Number(Number),
    Bool(Bool),
    UnOp(Operator, Box<AstNode>),
    BinOp(Box<AstNode>, Operator, Box<AstNode>),
    VarCreate(String, Box<AstNode>),
    VarAssign(String, Box<AstNode>),
    VarAccess(String),
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, String> {
    let mut tokens: Peekable<Iter<Token>> = tokens.iter().peekable();
    parse_createvar(&mut tokens)
}

pub fn parse_createvar(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    match tokens.peek() {
        Some(Token::Ident(ident)) => match ident.as_str() {
            "let" => {
                tokens.next();
                match tokens.next() {
                    Some(Token::Ident(var_name)) => {
                        if KEYWORDS.contains(&var_name.as_str()) {
                            return Err(format!("Expected identifier, found `{}`", var_name));
                        };
                        match tokens.next() {
                            Some(Token::Operator(Operator::Equals)) => Ok(AstNode::VarCreate(
                                var_name.clone(),
                                Box::new(parse_comparison(tokens)?),
                            )),
                            _ => Err("Expected `=`".to_string()),
                        }
                    }
                    _ => Err("Expected identifier".to_string()),
                }
            }
            _ => parse_comparison(tokens),
        },
        _ => parse_comparison(tokens),
    }
}

pub fn parse_comparison(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    if let Some(Token::Operator(Operator::Exclamation)) = tokens.peek() {
        tokens.next();
        return Ok(AstNode::UnOp(
            Operator::Exclamation,
            Box::new(parse_comparison(tokens)?),
        ));
    }

    let mut left_expr = parse_expr(tokens)?;
    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::EqualsEquals => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            Operator::ExclamationEquals => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            Operator::Greater => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            Operator::Less => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            Operator::GreaterEquals => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            Operator::LessEquals => {
                tokens.next();
                left_expr = AstNode::BinOp(Box::new(left_expr), *op, Box::new(parse_expr(tokens)?));
            }
            _ => break,
        }
    }
    Ok(left_expr)
}

pub fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    let mut left_term = parse_term(tokens)?;
    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::Plus => {
                tokens.next();
                left_term = AstNode::BinOp(Box::new(left_term), *op, Box::new(parse_term(tokens)?));
            }
            Operator::Minus => {
                tokens.next();
                left_term = AstNode::BinOp(Box::new(left_term), *op, Box::new(parse_term(tokens)?));
            }
            _ => break,
        }
    }
    Ok(left_term)
}
pub fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    let mut left_factor = parse_factor(tokens)?;

    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::Star => {
                tokens.next();
                left_factor =
                    AstNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            Operator::Slash => {
                tokens.next();
                left_factor =
                    AstNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            Operator::Percent => {
                tokens.next();
                left_factor =
                    AstNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            _ => break,
        }
    }
    Ok(left_factor)
}

pub fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    let factor = match tokens.peek() {
        Some(token) => match token {
            Token::Operator(op) => match op {
                Operator::Plus => {
                    tokens.next();
                    AstNode::UnOp(*op, Box::new(parse_factor(tokens)?))
                }
                Operator::Minus => {
                    tokens.next();
                    AstNode::UnOp(*op, Box::new(parse_factor(tokens)?))
                }
                _ => parse_power(tokens)?,
            },
            Token::Eof => return Err("Unexpected EOF".to_string()),
            _ => parse_power(tokens)?,
        },
        None => return Err("Syntax error".to_string()),
    };
    Ok(factor)
}

pub fn parse_power(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    let mut atom = parse_atom(tokens)?;

    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::StarStar => {
                tokens.next();
                atom = AstNode::BinOp(Box::new(atom), *op, Box::new(parse_factor(tokens)?));
            }
            _ => break,
        }
    }
    Ok(atom)
}

pub fn parse_atom(tokens: &mut Peekable<Iter<Token>>) -> Result<AstNode, String> {
    let atom = match tokens.next() {
        Some(token) => match token {
            Token::Number(n) => AstNode::Number(*n),
            Token::Ident(ident) => {
                if let Some(Token::Operator(Operator::Equals)) = tokens.peek() {
                    tokens.next();
                    if KEYWORDS.contains(&ident.as_str()) {
                        return Err(format!("Expected identifier, found `{}`", ident));
                    };
                    return Ok(AstNode::VarAssign(
                        ident.clone(),
                        Box::new(parse_comparison(tokens)?),
                    ));
                };
                match ident.as_str() {
                    "inf" => AstNode::Number(Number::Float(f64::INFINITY)),
                    "NaN" => AstNode::Number(Number::Float(f64::NAN)),
                    "true" => AstNode::Bool(Bool::True),
                    "false" => AstNode::Bool(Bool::False),
                    _ => AstNode::VarAccess(ident.clone()),
                }
            }
            Token::Operator(op) => match op {
                Operator::Parenthesis(paren) => match paren {
                    Parenthesis::LParen => {
                        let expr = parse_comparison(tokens)?;
                        let token = tokens.next();
                        match token {
                            Some(token) => match token {
                                Token::Operator(Operator::Parenthesis(Parenthesis::RParen)) => expr,
                                Token::Eof => return Err("Expected `)`, found EOF".to_string()),
                                _ => return Err(format!("Expected `)`, found: {:?}", token)),
                            },
                            _ => unreachable!(),
                        }
                    }
                    Parenthesis::RParen => return Err("Expected expression".to_string()),
                    _ => return Err(format!("Unexpected token: {:?}", token)),
                },
                _ => return Err(format!("Syntax error, {}:{}", line!(), column!())),
            },
            Token::Eof => return Err("Unexpected EOF".to_string()),
        },
        None => return Err(format!("Syntax error, {}:{}", line!(), column!())),
    };

    Ok(atom)
}

use std::{iter::Peekable, slice::Iter};

use crate::stdlib::number::Number;

use super::lexer::{Operator, Parenthesis, Token};

const KEYWORDS: [&str; 3] = ["let", "inf", "NaN"];

#[derive(Debug)]
pub enum ASTNode {
    Number(Number),
    UnOp(Operator, Box<ASTNode>),
    BinOp(Box<ASTNode>, Operator, Box<ASTNode>),
    VarCreate(String, Box<ASTNode>),
    VarAssign(String, Box<ASTNode>),
    VarAccess(String),
}

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut tokens: Peekable<Iter<Token>> = tokens.iter().peekable();
    parse_createvar(&mut tokens)
}

pub fn parse_createvar(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
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
                            Some(Token::Operator(Operator::Equals)) => {
                                return Ok(ASTNode::VarCreate(
                                    var_name.clone(),
                                    Box::new(parse_expr(tokens)?),
                                ));
                            }
                            _ => return Err(format!("Expected `=`")),
                        }
                    }
                    _ => return Err(format!("Expected identifier")),
                }
            }
            _ => parse_expr(tokens),
        },
        _ => parse_expr(tokens),
    }
}

pub fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let mut left_term = parse_term(tokens)?;
    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::Plus => {
                tokens.next();
                left_term = ASTNode::BinOp(Box::new(left_term), *op, Box::new(parse_term(tokens)?));
            }
            Operator::Minus => {
                tokens.next();
                left_term = ASTNode::BinOp(Box::new(left_term), *op, Box::new(parse_term(tokens)?));
            }
            _ => break,
        }
    }
    Ok(left_term)
}
pub fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let mut left_factor = parse_factor(tokens)?;

    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::Star => {
                tokens.next();
                left_factor =
                    ASTNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            Operator::Slash => {
                tokens.next();
                left_factor =
                    ASTNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            Operator::Percent => {
                tokens.next();
                left_factor =
                    ASTNode::BinOp(Box::new(left_factor), *op, Box::new(parse_factor(tokens)?));
            }
            _ => break,
        }
    }
    Ok(left_factor)
}

pub fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let factor = match tokens.peek() {
        Some(token) => match token {
            Token::Operator(op) => match op {
                Operator::Plus => {
                    tokens.next();
                    ASTNode::UnOp(*op, Box::new(parse_factor(tokens)?))
                }
                Operator::Minus => {
                    tokens.next();
                    ASTNode::UnOp(*op, Box::new(parse_factor(tokens)?))
                }
                _ => parse_power(tokens)?,
            },
            _ => parse_power(tokens)?,
            Token::Eof => return Err(format!("Unexpected EOF")),
        },
        None => return Err(format!("Syntax error")),
    };
    Ok(factor)
}

pub fn parse_power(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let mut atom = parse_atom(tokens)?;

    while let Some(Token::Operator(op)) = tokens.peek() {
        match op {
            Operator::StarStar => {
                tokens.next();
                atom = ASTNode::BinOp(Box::new(atom), *op, Box::new(parse_factor(tokens)?));
            }
            _ => break,
        }
    }
    Ok(atom)
}

pub fn parse_atom(tokens: &mut Peekable<Iter<Token>>) -> Result<ASTNode, String> {
    let atom = match tokens.next() {
        Some(token) => match token {
            Token::Number(n) => ASTNode::Number(*n),
            Token::Ident(ident) => {
                if let Some(Token::Operator(Operator::Equals)) = tokens.peek() {
                    tokens.next();
                    if KEYWORDS.contains(&ident.as_str()) {
                        return Err(format!("Expected identifier, found `{}`", ident));
                    };
                    return Ok(ASTNode::VarAssign(
                        ident.clone(),
                        Box::new(parse_expr(tokens)?),
                    ));
                };
                match ident.as_str() {
                    "inf" => ASTNode::Number(Number::Float(f64::INFINITY)),
                    "NaN" => ASTNode::Number(Number::Float(f64::NAN)),
                    _ => ASTNode::VarAccess(ident.clone()),
                }
            }
            Token::Operator(op) => match op {
                Operator::Parenthesis(paren) => match paren {
                    Parenthesis::LParen => {
                        let expr = parse_expr(tokens)?;
                        let token = tokens.next();
                        match token {
                            Some(token) => match token {
                                Token::Operator(Operator::Parenthesis(Parenthesis::RParen)) => expr,
                                Token::Eof => return Err(format!("Expected `)`, found EOF")),
                                _ => return Err(format!("Expected `)`, found: {:?}", token)),
                            },
                            _ => unreachable!(),
                        }
                    }
                    Parenthesis::RParen => return Err(format!("Expected expression")),
                    _ => return Err(format!("Unexpected token: {:?}", token)),
                },
                _ => return Err(format!("Syntax error, {}:{}", line!(), column!())),
            },
            Token::Eof => return Err(format!("Unexpected EOF")),
        },
        None => return Err(format!("Syntax error, {}:{}", line!(), column!())),
    };

    Ok(atom)
}

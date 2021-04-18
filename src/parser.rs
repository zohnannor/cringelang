use std::{iter::Peekable, slice::Iter};

use crate::{
    lexer::{Operator, Parentheses, Token},
    number::Number,
};

#[derive(Debug)]
pub enum ASTNode {
    Number(Number),
    Ident(String),
    UnOp(Operator, Box<ASTNode>),
    BinOp(Box<ASTNode>, Operator, Box<ASTNode>),
}

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut tokens: Peekable<Iter<Token>> = tokens.iter().peekable();
    parse_expr(&mut tokens)
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
            Token::Ident(ident) => ASTNode::Ident(ident.clone()),
            Token::Operator(op) => match op {
                Operator::Parentheses(paren) => match paren {
                    Parentheses::LParen => {
                        let expr = parse_expr(tokens)?;
                        let token = tokens.next();
                        match token {
                            Some(token) => match token {
                                Token::Operator(Operator::Parentheses(Parentheses::RParen)) => expr,
                                Token::Eof => return Err(format!("Unexpected EOF")),
                                _ => return Err(format!("Expected `)`, found: {:?}", token)),
                            },
                            None => return Err(format!("Unexpected asdadas")),
                        }
                    }
                    Parentheses::RParen => {
                        return Err(format!("Syntax error, {}:{}", line!(), column!()))
                    }
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

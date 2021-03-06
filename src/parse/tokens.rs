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
    Exclamation,
    Pipe,
    Ampersand,
    Caret,
    Greater,
    Less,
    GreaterGreater,
    LessLess,
    GreaterEquals,
    LessEquals,
    EqualsEquals,
    ExclamationEquals,
    Dot,
    Parenthesis(Parenthesis),
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Char(char),
    String(String),
    Ident(String),
    Operator(Operator),
    Eof,
}

use std::fmt;
use super::symbols::Symbols;
use super::number::Number;
use super::keywords::Keyword;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenTypes {
    Unknown,

    Number,
    Symbol,
    Identifier,
    Paren,
    Keyword,
}

// --- --- --- --- --- ---

#[derive(PartialEq)]
pub enum Token {
    Number(Number),
    Symbol(Symbols),
    Paren(Symbols),
    Identi(String),
    Keyword(Keyword),

    Divider,
}
pub type TokenVec = Vec<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(_)  => write!(f, "token: Number"),
            Token::Symbol(_)  => write!(f, "token: Symbol"),
            Token::Paren(_)   => write!(f, "token: Paren"),
            Token::Identi(_)  => write!(f, "token: Identifier"),
            Token::Keyword(_) => write!(f, "token: Keyword"),
            Token::Divider => write!(f, "token: Divider"),
        }
    }
}
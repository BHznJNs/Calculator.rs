use std::collections::VecDeque;
use std::fmt;

use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::value::number::Number;
use crate::public::compile_time::keywords::Keywords;
use crate::public::value::value::ValueTypes;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenTypes {
    Unknown,

    Number,
    Symbol,
    Identifier,
    Paren,
    Keywords,

    Annotation,
}

// --- --- --- --- --- ---

#[derive(PartialEq)]
pub enum Token {
    Number(Number),
    Symbol(Symbols),
    Paren(Parens),
    Identi(String),
    Keywords(Keywords),

    Divider,
    Annotation(ValueTypes),
}
pub type TokenVec = VecDeque<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(_)   => write!(f, "token: Number"),
            Token::Symbol(_)   => write!(f, "token: Symbol"),
            Token::Paren(_)    => write!(f, "token: Paren"),
            Token::Identi(_)   => write!(f, "token: Identifier"),
            Token::Keywords(_) => write!(f, "token: Keywords"),
            Token::Divider     => write!(f, "token: Divider"),
            Token::Annotation(_)  => write!(f, "token: Annotation"),
        }
    }
}
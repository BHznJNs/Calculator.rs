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
    String,
    Symbol,
    Identifier,
    Paren,
    Keywords,

    Annotation,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Token {
    Number(Number),
    String(String),
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
            Token::Number(num)     => write!(f, "Number: {}", num),
            Token::String(str)     => write!(f, "String: {}", str),
            Token::Symbol(sym)    => write!(f, "Symbol: {}", sym),
            Token::Paren(par)      => write!(f, "Paren: {:#?}", par),
            Token::Identi(ide)     => write!(f, "Identifier: {}", ide),
            Token::Keywords(key) => write!(f, "Keywords: {}", key),
            Token::Divider                  => write!(f, "Divider"),
            Token::Annotation(typ)  => write!(f, "Annotation: {}", typ),
        }
    }
}
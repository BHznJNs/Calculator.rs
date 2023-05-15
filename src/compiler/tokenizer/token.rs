use std::collections::VecDeque;
use std::fmt;

use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::value::number::Number;
use crate::public::compile_time::keywords::Keywords;
use crate::public::value::value::ValueType;

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
    Annotation(ValueType),
}
pub type TokenVec = VecDeque<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_display =
        match self {
            Token::Number(num)     => format!("Number: {}", num),
            Token::String(str)     => format!("String: {}", str),
            Token::Symbol(sym)    => format!("Symbol: {}", sym),
            Token::Paren(par)      => format!("Paren: {:#?}", par),
            Token::Identi(ide)     => format!("Identifier: {}", ide),
            Token::Keywords(key) => format!("Keywords: {}", key),
            Token::Divider                  => format!("Divider"),
            Token::Annotation(typ)  => format!("Annotation: {}", typ),
        };
        write!(f, "Token({})", token_display)
    }
}
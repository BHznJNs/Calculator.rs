use std::collections::VecDeque;
use std::fmt;

use crate::public::compile_time::keywords::Keyword;
use crate::public::compile_time::parens::Paren;
use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;
use crate::public::value::value::ValueType;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    Unknown,

    Number,
    String,
    Symbol,
    Identifier,
    Paren,
    Keyword,

    Annotation,
}

// --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub enum Token {
    Number(Number),
    String(String),
    Symbol(Symbols),
    Paren(Paren),
    Identi(String),
    Keyword(Keyword),

    Divider,
    Annotation(ValueType),
}
pub type TokenVec = VecDeque<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_display = match self {
            Token::Number(num) => format!("Number: {}", num),
            Token::String(str) => format!("String: {}", str),
            Token::Symbol(sym) => format!("Symbol: {}", sym),
            Token::Paren(par) => format!("Paren: {:#?}", par),
            Token::Identi(ide) => format!("Identifier: {}", ide),
            Token::Keyword(key) => format!("Keywords: {}", key),
            Token::Divider => format!("Divider"),
            Token::Annotation(type__) => format!("Annotation: {}", type__),
        };
        write!(f, "Token({})", token_display)
    }
}

use std::collections::VecDeque;
use std::fmt;

use crate::public::compile_time::dividers::Divider;
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

    Divider(Divider),
    Annotation(ValueType),
}
pub type TokenVec = VecDeque<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_content_display = match self {
            Self::Number(num) => format!("Number: {}", num),
            Self::String(str) => format!("String: {}", str),
            Self::Symbol(sym) => format!("Symbol: {}", sym),
            Self::Paren(par) => format!("Paren: {:#?}", par),
            Self::Identi(ide) => format!("Identifier: {}", ide),
            Self::Keyword(key) => format!("Keywords: {}", key),
            Self::Divider(div) => format!("Divider: {:?}", div),
            Self::Annotation(type__) => format!("Annotation: {}", type__),
        };
        write!(f, "Token({})", token_content_display)
    }
}

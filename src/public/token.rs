use std::fmt;
use super::types::Types;
use super::symbols::Symbols;
use super::number::Number;

#[derive(Default)]
pub struct Token {
    pub type__: Types,
    pub number: Number,
    pub symbol: Symbols,
    pub identi: Option<String>,
}
pub type TokenVec = Vec<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type__ {
            Types::Number => write!(f, "{}", self.number),
            Types::Symbol => write!(f, "{}", self.symbol),
            Types::Paren  => write!(f, "{}", self.symbol),
            Types::Identifier => write!(f, "{:#?}", self.identi),
            Types::Unknown => write!(f, "Unknown"),
        }
    }
}

pub trait Overloaded<T> {
    fn create(type__: Types, value: T) -> Self;
}
impl Overloaded<Number> for Token {
    fn create(type__: Types, value: Number) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.number = value;
        return default_token
    }
}
impl Overloaded<Symbols> for Token {
    fn create(type__: Types, value: Symbols) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.symbol = value;
        return default_token
    }
}
impl Overloaded<String> for Token {
    fn create(type__: Types, value: String) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.identi = Some(value);
        return default_token
    }
}
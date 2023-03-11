use std::fmt;
use super::symbols::Symbols;
use super::number::Number;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenTypes {
    Unknown,

    Number,
    Symbol,
    Identifier,
    Paren,
}

impl Default for TokenTypes {
    fn default() -> Self {
        return TokenTypes::Unknown
    }
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTypes::Unknown => write!(f, "Unknown"),
            TokenTypes::Number => write!(f, "Number"),
            TokenTypes::Symbol => write!(f, "Symbol"),
            TokenTypes::Identifier => write!(f, "Identifier"),
            TokenTypes::Paren  => write!(f, "Parentheses"),
        }
    }
}

// --- --- --- --- --- ---

#[derive(Default, Clone)]
pub struct Token {
    pub type__: TokenTypes,
    pub number: Number,
    pub symbol: Symbols,
    pub identi: Option<String>,
}
pub type TokenVec = Vec<Token>;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type__ {
            TokenTypes::Number => write!(f, "{}", self.number),
            TokenTypes::Symbol => write!(f, "{}", self.symbol),
            TokenTypes::Paren  => write!(f, "{}", self.symbol),
            TokenTypes::Identifier => write!(f, "{:#?}", self.identi),
            TokenTypes::Unknown => write!(f, "Unknown"),
        }
    }
}

pub trait Overloaded<T> {
    fn create(type__: TokenTypes, value: T) -> Self;
}
impl Overloaded<Number> for Token {
    fn create(type__: TokenTypes, value: Number) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.number = value;
        return default_token
    }
}
impl Overloaded<Symbols> for Token {
    fn create(type__: TokenTypes, value: Symbols) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.symbol = value;
        return default_token
    }
}
impl Overloaded<String> for Token {
    fn create(type__: TokenTypes, value: String) -> Self {
        let mut default_token = Token::default();
        default_token.type__ = type__;
        default_token.identi = Some(value);
        return default_token
    }
}
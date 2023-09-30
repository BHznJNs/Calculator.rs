use std::ops::{RangeFull, RangeFrom, RangeTo};

use crossterm::style::{StyledContent, Stylize};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Unknown,

    Identifier,
    Keyword,
    Annotation,

    Paren,
    Symbol,
    Divider,
    Comment,

    Number,
    String,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub type__: TokenType,
    pub content: String,
}

pub type TokenVec = Vec<Token>;

impl Token {
    pub fn new(type__: TokenType, content: String) -> Self {
        Self { type__, content }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    fn colored(type__: TokenType, text: &str) -> StyledContent<&str> {
        match type__ {
            TokenType::Unknown => text.white().on_dark_red(),

            TokenType::Identifier => text.underlined(),
            TokenType::Keyword => text.dark_cyan(),
            TokenType::Annotation => text.red(),

            TokenType::Paren | TokenType::Symbol => text.white(),
            TokenType::Divider => text.dim(),
            TokenType::Comment => text.green().dim(),

            TokenType::Number => text.yellow(),
            TokenType::String => text.dark_yellow(),
        }
    }
}

pub trait TokenSlicing<R> {
    fn get(&self, range: R) -> StyledContent<&str>;
}
impl TokenSlicing<RangeFrom<usize>> for Token {
    fn get(&self, range: RangeFrom<usize>) -> StyledContent<&str> {
        Self::colored(self.type__, &self.content[range])
    }
}
impl TokenSlicing<RangeTo<usize>> for Token {
    fn get(&self, range: RangeTo<usize>) -> StyledContent<&str> {
        Self::colored(self.type__, &self.content[range])
    }
}
impl TokenSlicing<RangeFull> for Token {
    fn get(&self, range: RangeFull) -> StyledContent<&str> {
        Self::colored(self.type__, &self.content[range])
    }
}

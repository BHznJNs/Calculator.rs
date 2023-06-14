use std::{collections::VecDeque, ops::Range};

use crossterm::style::{Stylize, StyledContent};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Unknown,

    Number,
    String,
    Symbol,
    Paren,
    Identifier,
    Keyword,

    Divider,
    Annotation,
    Comment,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TextType {
    Hint,

    Variable,
    Keyword,
    Annotation,

    Didider,
    Comment,

    NumberLiteral,
    StringLiteral,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub type__: TextType,
    pub content: String,
}

pub type TokenVec = VecDeque<Token>;

impl Token {
    pub fn new(type__: TextType, content: String) -> Self {
        Token { type__, content }
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn colored(&self, range: Range<usize>) -> StyledContent<&str> {
        let text = &self.content[range];

        match self.type__ {
            TextType::Hint => text.dim(),

            TextType::Variable => text.underlined(),
            TextType::Keyword => text.dark_cyan(),
            TextType::Annotation => text.red(),

            TextType::Didider => text.white(),
            TextType::Comment => text.green().dim(),

            TextType::NumberLiteral => text.yellow(),
            TextType::StringLiteral => text.dark_yellow(),
        }
    }
}

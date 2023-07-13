use std::{collections::VecDeque, ops::Range};

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TextType {
    Hint,

    Variable,
    Keyword,
    Annotation,

    Paren,
    Symbol,
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

pub type TokenVec = Vec<Token>;

impl Token {
    pub fn new(type__: TextType, content: String) -> Self {
        Token { type__, content }
    }
    pub fn len(&self) -> usize {
        self.content.chars().count()
    }

    pub fn colored(&self, range: Range<usize>) -> StyledContent<&str> {
        let text = &self.content[range];

        match self.type__ {
            TextType::Hint => text.dim(),

            TextType::Variable => text.underlined(),
            TextType::Keyword => text.dark_cyan(),
            TextType::Annotation => text.red(),

            TextType::Paren | TextType::Symbol => text.white(),
            TextType::Didider => text.dim(),
            TextType::Comment => text.green().dim(),

            TextType::NumberLiteral => text.yellow(),
            TextType::StringLiteral => text.dark_yellow(),
        }
    }
}

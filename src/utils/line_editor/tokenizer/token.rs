use std::collections::VecDeque;

use crate::utils::line_editor::terminal::TextType;

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
}

use std::fmt::Display;

use crossterm::style::Stylize;

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

impl TextType {
    pub fn match_tx_type(text: &str, type__: TextType) -> String {
        match type__ {
            TextType::Hint => format!("{}", text.dim()),

            TextType::Variable => format!("{}", text.underlined()),
            TextType::Keyword => format!("{}", text.dark_cyan()),
            TextType::Annotation => format!("{}", text.red()),

            TextType::Didider => format!("{}", text.white()),
            TextType::Comment => format!("{}", text.dark_green()),

            TextType::NumberLiteral => format!("{}", text.yellow()),
            TextType::StringLiteral => format!("{}", text.dark_yellow()),
        }
    }
}

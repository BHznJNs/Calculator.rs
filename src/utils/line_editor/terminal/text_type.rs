use crossterm::style::{StyledContent, Stylize};

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
    pub fn match_tx_type(text: &str, type__: TextType) -> StyledContent<&str> {
        match type__ {
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

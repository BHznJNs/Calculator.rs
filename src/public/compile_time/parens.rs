#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum Paren {
    // ()
    LeftParen,
    RightParen,
    // []
    LeftBracket,
    RightBracket,
    // {}
    LeftBrace,
    RightBrace,
}

impl From<char> for Paren {
    fn from(value: char) -> Self {
        match value {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '[' => Self::LeftBracket,
            ']' => Self::RightBracket,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            _ => unreachable!(),
        }
    }
}

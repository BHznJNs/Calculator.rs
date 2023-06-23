#[derive(PartialEq, Debug, Clone, Copy)]
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

impl Paren {
    pub fn from(ch: char) -> Self {
        match ch {
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '[' => Self::LeftBracket,
            ']' => Self::RightBracket,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            _ => unreachable!()
        }
    }
}

use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Symbols {
    NotASymbol,

    Plus,
    Minus,
    Multiply,
    Divide,

    LeftParen,
    RightParen,
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbols::NotASymbol => write!(f, "NotASymbol"),

            Symbols::Plus     => write!(f, "Plus"),
            Symbols::Minus    => write!(f, "Minus"),
            Symbols::Multiply => write!(f, "Multiply"),
            Symbols::Divide   => write!(f, "Divide"),
        
            Symbols::LeftParen  => write!(f, "LeftParen"),
            Symbols::RightParen => write!(f, "RightParen"),
        }
    }
}
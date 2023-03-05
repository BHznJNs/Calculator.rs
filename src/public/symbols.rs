use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Symbols {
    NotASymbol,

    Plus,
    Minus,
    Multiply,
    Divide,
    Power,

    LeftParen,
    RightParen,
}

impl Default for Symbols {
    fn default() -> Self {
        return Symbols::NotASymbol
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbols::NotASymbol => write!(f, "NotASymbol"),

            Symbols::Plus     => write!(f, "Plus"),
            Symbols::Minus    => write!(f, "Minus"),
            Symbols::Multiply => write!(f, "Multiply"),
            Symbols::Divide   => write!(f, "Divide"),
            Symbols::Power    => write!(f, "Power"),
        
            Symbols::LeftParen  => write!(f, "LeftParen"),
            Symbols::RightParen => write!(f, "RightParen"),
        }
    }
}
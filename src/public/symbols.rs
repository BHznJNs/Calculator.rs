use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Symbols {
    NotASymbol,

    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Equal,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Default for Symbols {
    fn default() -> Self {
        return Symbols::NotASymbol
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbols::NotASymbol => write!(f, "Not A Symbol"),

            Symbols::Plus     => write!(f, "Plus"),
            Symbols::Minus    => write!(f, "Minus"),
            Symbols::Multiply => write!(f, "Multiply"),
            Symbols::Divide   => write!(f, "Divide"),
            Symbols::Power    => write!(f, "Power"),
            Symbols::Equal    => write!(f, "Equal"),
        
            Symbols::LeftParen  => write!(f, "LeftParen"),
            Symbols::RightParen => write!(f, "RightParen"),
            Symbols::LeftBrace  => write!(f, "LeftBrace"),
            Symbols::RightBrace => write!(f, "RightBrace"),
        }
    }
}
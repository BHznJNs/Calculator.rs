use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Types {
    Unknown,

    Number,
    Identifier,
    Symbol,
    Paren,
}

impl Default for Types {
    fn default() -> Self {
        return Types::Unknown
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Types::Unknown => write!(f, "Unknown"),
            Types::Number => write!(f, "Number"),
            Types::Identifier => write!(f, "Identifier"),
            Types::Symbol => write!(f, "Symbol"),
            Types::Paren  => write!(f, "Parentheses"),
        }
    }
}
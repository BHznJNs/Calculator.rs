use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Types {
    Number,
    Symbol,
    Paren,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Types::Number => write!(f, "number"),
            Types::Symbol => write!(f, "symbol"),
            Types::Paren  => write!(f, "parentheses"),
        }
    }
}
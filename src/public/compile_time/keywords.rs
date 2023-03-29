use std::fmt;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Keywords {
    Out,
    Fn,

    For,
    If,

    Continue,
    Break,

    Import,
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keywords::Out => write!(f, "out"),
            Keywords::Fn  => write!(f, "fn" ),
            Keywords::For => write!(f, "for"),
            Keywords::If  => write!(f, "if" ),
            Keywords::Continue => write!(f, "continue"),
            Keywords::Break => write!(f, "break"),
            Keywords::Import => write!(f, "import"),
        }
    }
}

pub const KEYWORDS_ENUM: [Keywords; 7] = [
    Keywords::Out,
    Keywords::Fn,
    Keywords::For,
    Keywords::If,
    Keywords::Continue,
    Keywords::Break,
    Keywords::Import,
];

pub const KEYWORDS: [&str; 7] = [
    "out",
    "fn",
    "for",
    "if",
    "ctn",
    "brk",

    "import",
];
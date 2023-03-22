use std::fmt;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Keyword {
    Out,
    For,
    If,

    Continue,
    Break,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Out => write!(f, "keyword: out"),
            Keyword::For => write!(f, "keyword: for"),
            Keyword::If  => write!(f, "keyword: if"),
            Keyword::Continue => write!(f, "keyword: continue"),
            Keyword::Break => write!(f, "keyword: break"),
        }
    }
}

pub const KEYWORDS_ENUM: [Keyword; 5] = [
    Keyword::Out,
    Keyword::For,
    Keyword::If,
    Keyword::Continue,
    Keyword::Break,
];

pub const KEYWORDS: [&str; 5] = [
    "out",
    "for",
    "if",
    "ctn",
    "brk"
];
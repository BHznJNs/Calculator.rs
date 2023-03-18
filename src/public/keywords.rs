use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Keyword {
    Out,
    Loop,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Out  => write!(f, "keyword: out"),
            Keyword::Loop => write!(f, "keyword: loop"),
        }
    }
}

pub const KEYWORDS_ENUM: [Keyword; 2] = [
    Keyword::Out,
    Keyword::Loop,
];

pub const KEYWORDS: [&str; 2] = [
    "out",
    "loop",
];
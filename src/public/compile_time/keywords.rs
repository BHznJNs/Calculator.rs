use std::fmt;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Keywords {
    Out,

    For,
    If,

    Continue,
    Break,

    Import,

    Function,
    Class, New,
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keywords::Out => write!(f, "out"),
            Keywords::For => write!(f, "for"),
            Keywords::If  => write!(f, "if" ),
            Keywords::Continue => write!(f, "continue"),
            Keywords::Break    => write!(f, "break"),
            Keywords::Import   => write!(f, "import"),
            Keywords::Function => write!(f, "function"),
            Keywords::Class    => write!(f, "class"),
            Keywords::New      => write!(f, "new"),
        }
    }
}

pub const KEYWORDS_ENUM: [Keywords; 9] = [
    Keywords::Out,

    Keywords::For,
    Keywords::If,

    Keywords::Continue,
    Keywords::Break,

    Keywords::Import,

    Keywords::Function,
    Keywords::Class,
    Keywords::New,
];

pub const KEYWORDS: [&str; 9] = [
    "out",

    "for",
    "if",

    "ctn",
    "brk",

    "import",

    "fn",
    "cls",
    "new",
];
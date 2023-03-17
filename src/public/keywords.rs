#[derive(PartialEq, Clone, Copy)]
pub enum Keyword {
    Out,
    Loop,
}

pub const KEYWORDS_ENUM: [Keyword; 2] = [
    Keyword::Out,
    Keyword::Loop,
];

pub const KEYWORDS: [&str; 2] = [
    "out",
    "loop",
];
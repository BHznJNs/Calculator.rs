#[derive(PartialEq)]
pub enum Keyword {
    Out,
    Loop,
}

pub const KEYWORDS: [&str; 2] = [
    "out",
    "loop",
];
use std::fmt;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Keyword {
    Out,

    For,
    If,

    Continue,
    Break,

    Import,

    Function,
    Class,
    New,
}

impl Keyword {
    pub fn is_keyword(word: &str) -> Option<Keyword> {
        // check is keyword
        let keyword: Keyword;

        let mut index = 0;
        while index < KEYWORD_PAIRS.len() {
            let current = KEYWORD_PAIRS[index];

            if word.eq(current.0) {
                keyword = current.1;
                return Some(keyword);
            }
            index += 1;
        }
        return None;
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Out => write!(f, "out"),
            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Break => write!(f, "break"),
            Keyword::Import => write!(f, "import"),
            Keyword::Function => write!(f, "function"),
            Keyword::Class => write!(f, "class"),
            Keyword::New => write!(f, "new"),
        }
    }
}

pub const KEYWORD_PAIRS: [(&'static str, Keyword); 9] = [
    ("out", Keyword::Out),
    ("for", Keyword::For),
    ("if", Keyword::If),
    ("ctn", Keyword::Continue),
    ("brk", Keyword::Break),
    ("import", Keyword::Import),
    ("fn", Keyword::Function),
    ("cl", Keyword::Class),
    ("new", Keyword::New),
];

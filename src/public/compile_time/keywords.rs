use std::{fmt, collections::HashMap};

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
    Global,
}

static mut KEYWORD_PAIRS: Option<HashMap<&'static str, Keyword>> = None;

impl Keyword {
    pub fn is_keyword(word: &str) -> Option<Self> {
        if unsafe { KEYWORD_PAIRS.is_none() } {
            // init global var
            unsafe { KEYWORD_PAIRS = Some(HashMap::from([
                ("out", Keyword::Out),
                ("for", Keyword::For),
                ("if", Keyword::If),
                ("ctn", Keyword::Continue),
                ("brk", Keyword::Break),
                ("import", Keyword::Import),
                ("fn", Keyword::Function),
                ("cl", Keyword::Class),
                ("class", Keyword::Class),
                ("new", Keyword::New),
                ("glo", Keyword::Global),

                // full version of keywords
                ("continue", Keyword::Continue),
                ("break", Keyword::Break),
                ("function", Keyword::Function),
                ("global", Keyword::Global),
            ])) };
        }
        let pair_map = unsafe { KEYWORD_PAIRS.as_ref().unwrap() };
        let keyword = pair_map.get(word)?;
        return Some(*keyword);
    }
}

// pub const KEYWORD_PAIRS: [(&str, Keyword); 10] = [
//     ("out", Keyword::Out),
//     ("for", Keyword::For),
//     ("if", Keyword::If),
//     ("ctn", Keyword::Continue),
//     ("brk", Keyword::Break),
//     ("import", Keyword::Import),
//     ("fn", Keyword::Function),
//     ("cl", Keyword::Class),
//     ("new", Keyword::New),
//     ("glo", Keyword::Global),
// ];

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
            Keyword::Global => write!(f, "global"),
        }
    }
}

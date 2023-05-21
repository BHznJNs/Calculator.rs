#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

const COLOR_VALUE_PAIRS: [(&'static str, &'static str); 16] = [
    // front | background
    ("30", "40"),
    ("31", "41"),
    ("32", "42"),
    ("33", "43"),
    ("34", "44"),
    ("35", "45"),
    ("36", "46"),
    ("37", "47"),
    ("90", "100"),
    ("91", "101"),
    ("92", "102"),
    ("93", "103"),
    ("94", "104"),
    ("95", "105"),
    ("96", "106"),
    ("97", "107"),
];

impl Color {
    pub fn get_fg_str(&self) -> &'static str {
        COLOR_VALUE_PAIRS[*self as usize].0
    }
    pub fn get_bg_str(&self) -> &'static str {
        COLOR_VALUE_PAIRS[*self as usize].1
    }
}

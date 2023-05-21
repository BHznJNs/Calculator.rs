#[derive(Clone, Copy)]
pub enum Style {
    Normal,
    Bold,
    Darken,
    Italic,
    Underlined,
    Deleted,
}
const STYLE_MAP: [&'static str; 6] = ["0", "1", "2", "3", "4", "9"];

impl Style {
    pub fn get_str(&self) -> &'static str {
        STYLE_MAP[*self as usize]
    }
}

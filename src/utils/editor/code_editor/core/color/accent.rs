pub enum AccentColor {
    Red,
    Blue,
    DarkRed,
    DarkBlue,
    DarkGrey,
    DarkCyan,
    DarkYellow,
    DarkMagenta,
}
impl From<&str> for AccentColor {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "dark_red" => Self::DarkRed,
            "dark_blue" => Self::DarkBlue,
            "dark_grey" | "dark_gray" => Self::DarkGrey,
            "dark_cyan" => Self::DarkCyan,
            "dark_yellow" => Self::DarkYellow,
            "dark_magenta" => Self::DarkMagenta,
            _ => Self::Red,
        }
    }
}
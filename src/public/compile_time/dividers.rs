#[derive(PartialEq, Clone, Debug)]
pub enum Divider {
    Comma,     // ','
    Colon,     // ':'
    Semicolon, // ';'
}

impl From<char> for Divider {
    fn from(value: char) -> Self {
        match value {
            ',' => Self::Comma,
            ':' => Self::Colon,
            ';' => Self::Semicolon,
            _ => unreachable!(),
        }
    }
}

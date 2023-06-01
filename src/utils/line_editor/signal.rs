#[derive(Debug)]
pub enum Signal {
    NewLine(String),
    Interrupt,
    NonASCII,
}

// [a-z A-Z _ Non-ASCII]
pub fn is_identi_ascii(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || !ch.is_ascii()
}
// '1' -> 1
pub fn ascii_to_num(ch: char) -> i64 {
    const NUM_ASCII_START: u8 = 48;
    return ((ch as u8) - NUM_ASCII_START) as i64;
}

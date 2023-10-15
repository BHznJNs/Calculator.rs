pub fn char_converter(ch: char) -> char {
    let result: u8 = match ch {
        '\"' => 34,    // '\"'
        '\'' => 39,    // '\''
        '\\' => 92,    // '\\'
        'a' => 7,      // '\a'
        'b' => 8,      // '\b'
        'n' => 10,     // '\n'
        'r' => 13,     // '\r'
        's' => 32,     // '\s'
        't' => 9,      // '\t'
        'v' => 11,     // '\v'
        _ => ch as u8, // for other: return itself
    };
    return result as char;
}

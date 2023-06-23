pub fn char_converter(ch: char) -> Result<char, ()> {
    let result: u8 = match ch {
        '\"' => 34,  // '\"'
        '\'' => 39,  // '\''
        '\\' => 92,  // '\\'
        'a' => 7,   // '\a'
        'b' => 8,   // '\b'
        'n' => 10, // '\n'
        'r' => 13, // '\r'
        's' => 32, // '\s'
        't' => 9,  // '\t'
        'v' => 11, // '\v'
        _ => return Ok(ch),   // for other: return itself
    };
    Ok(result as char)
}

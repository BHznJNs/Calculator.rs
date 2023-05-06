pub fn char_converter(ch: u8) -> Result<u8, ()> {
    let result =
    match ch {
        34  => 34, // '\"'
        39  => 39, // '\''
        92  => 92, // '\\'
        97  => 7 , // '\a'
        98  => 8 , // '\b'
        110 => 10, // '\n'
        114 => 13, // '\r'
        115 => 32, // '\s'
        116 => 9,  // '\t'
        118 => 11, // '\v'
        _ => {
            println!("Invalid escape character: '{}'.", ch as char);
            return Err(())
        }
    };
    Ok(result)
}
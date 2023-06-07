// a-z A-Z _
pub fn is_identi_ascii(ascii: u8) -> bool {
    const UNDERLINE_ASCII: u8 = 95;
    return ascii.is_ascii_alphabetic() || ascii == UNDERLINE_ASCII;
}
// '1' -> 1
pub fn ascii_to_num(ascii: u8) -> u8 {
    const NUM_ASCII_START: u8 = 48;
    return ascii - NUM_ASCII_START;
}

pub const POINT_ASCII: u8 = 46;

pub const LEFT_PAREN_ASCII: u8 = 40; // (
pub const RIGHT_PAREN_ASCII: u8 = 41; // )
pub const LEFT_BRACKET_ASCII: u8 = 91; // [
pub const RIGHT_BRACKET_ASCII: u8 = 93; // ]
pub const LEFT_BRACE_ASCII: u8 = 123; // {
pub const RIGHT_BRACE_ASCII: u8 = 125; // }

pub const PLUS_ASCII: u8 = 43; // +
pub const MINUS_ASCII: u8 = 45; // -
pub const MULTIPLY_ASCII: u8 = 42; // *
pub const DIVIDE_ASCII: u8 = 47; // /
pub const POWER_ASCII: u8 = 94; // ^

pub const NOT_SYMBOL_ASCII: u8 = 33; // !
pub const LESS_THAN_ASCII: u8 = 60; // <
pub const MORE_THAN_ASCII: u8 = 62; // >
pub const EQUAL_ASCII: u8 = 61; // =

pub const SINGLE_QUOTE_ASCII: u8 = 39; // '''
pub const DOUBLE_QUOTE_ASCII: u8 = 34; // '"'
pub const BACKSLASH_ASCII: u8 = 92; // '\'

pub const SEMICOLON_ASCII: u8 = 59; // ;
pub const COMMA_ASCII: u8 = 44; // ,
pub const DOLLAR_ASCII: u8 = 36; // $
pub const NUMBER_SIGN_ASCII: u8 = 35; // #
pub const SPACE_ASCII: u8 = 32; // ' '
pub const TAB_ASCII: u8 = 9; // '\t'

pub const NULL_ASCII: u8 = 0; // '\0'

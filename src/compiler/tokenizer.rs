use super::types::Types;
use super::symbols::Symbols;

#[derive(Clone, Copy)]
pub struct Token {
    pub type__: Types,
    pub number: i32,
    pub symbol: Symbols,
}
// impl Copy for Token {}
// impl Clone for Token {
//     // 
// }

const NUM_ASCII_START: u8 = 48;
const NUM_ASCII_END  : u8 = 57;
fn is_numeric(char: u8) -> bool {
    if NUM_ASCII_START <= char && char <= NUM_ASCII_END {
        return true;
    } else {
        return false;
    }
}

pub fn tokenizer(source: String) -> Vec<Token> {
    let mut index = 0;
    let mut last_type = -1;
    let mut is_num_minus = false;
    let mut tokens = Vec::<Token>::new();
    let chars = source.as_bytes();
    let source_len = source.len();

    // println!("{} {}", source, source_len);

    while index < source_len {
        let mut current = chars[index];
        // println!("{} {}", index, current);

        if is_numeric(current) {
            let mut value: i32 = (current - NUM_ASCII_START) as i32;
            last_type = Types::Number as i8;

            index += 1;
            current = chars[index];

            while is_numeric(current) {
                value *= 10;
                value += (current - NUM_ASCII_START) as i32;
                index += 1;
                current = chars[index];
            }

            if is_num_minus {
                value = -value;
                is_num_minus = false;
            }

            tokens.push(Token {
                type__: Types::Number,
                number: value,
                symbol: Symbols::NotASymbol,
            });
            continue;
        }

        // --- --- --- --- --- ---

        const LEFT_PAREN_ASCII  : u8 = 40;
        const RIGHT_PAREN_ASCII : u8 = 41;
        
        const PLUS_ASCII     : u8 = 43;
        const MINUS_ASCII    : u8 = 45;
        const MULTIPLY_ASCII : u8 = 42;
        const DIVIDE_ASCII   : u8 = 47;
        
        const SPACE_ASCII  : u8 = 32;
        const RETURN_ASCII : u8 = 13;

        match current {
            LEFT_PAREN_ASCII => {
                last_type = Types::Paren as i8;
                tokens.push(Token {
                    type__: Types::Paren,
                    number: -1,
                    symbol: Symbols::LeftParen,
                });
            },
            RIGHT_PAREN_ASCII => {
                last_type = Types::Paren as i8;
                tokens.push(Token {
                    type__: Types::Paren,
                    number: -1,
                    symbol: Symbols::RightParen,
                });
            },

            PLUS_ASCII => {
                last_type = Types::Symbol as i8;
                tokens.push(Token {
                    type__: Types::Symbol,
                    number: -1, 
                    symbol: Symbols::Plus,
                });
            },
            MINUS_ASCII => {
                if last_type < 0 || last_type == Types::Symbol as i8 {
                    is_num_minus = true;
                } else {
                    last_type = Types::Symbol as i8;
                    tokens.push(Token {
                        type__: Types::Symbol,
                        number: -1, 
                        symbol: Symbols::Minus,
                    });
                }
            },
            MULTIPLY_ASCII => {
                last_type = Types::Symbol as i8;
                tokens.push(Token {
                    type__: Types::Symbol,
                    number: -1, 
                    symbol: Symbols::Multiply,
                });
            },
            DIVIDE_ASCII => {
                last_type = Types::Symbol as i8;
                tokens.push(Token {
                    type__: Types::Symbol,
                    number: -1, 
                    symbol: Symbols::Divide,
                });
            },
            SPACE_ASCII => {},
            RETURN_ASCII => { break; },
            _ => {
                panic!("Unknown token: {}", current);
            }
        }

        index += 1;
    }
    return tokens;
}
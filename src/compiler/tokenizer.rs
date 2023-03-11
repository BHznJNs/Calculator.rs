use crate::public::number::Number;
use crate::public::token::Overloaded;
use crate::public::token::Token;
use crate::public::token::TokenTypes;
use crate::public::token::TokenVec;
use crate::public::symbols::Symbols;

const NUM_ASCII_START: u8 = 48;
const POINT_ASCII: u8 = 46;
const UNDERLINE_ASCII: u8 = 95;

enum State {
    Int, Float
}

fn is_identi_ascii(ascii: u8) -> bool {
    return ascii.is_ascii_alphabetic() || ascii == UNDERLINE_ASCII;
}

fn ascii_to_num(ascii: u8) -> u8 {
    return ascii - NUM_ASCII_START
}

pub fn tokenizer(source: String) -> Result<TokenVec, ()> {
    let mut index = 0;

    // use to control if number is minus
    let mut last_type = TokenTypes::Unknown;
    let mut is_num_minus = false;

    let mut tokens = TokenVec::new();
    let chars = source.as_bytes();
    let source_len = source.len();

    while index < source_len {
        let mut current = chars[index];

        if !current.is_ascii() {
            println!("Please input ASCII characters!");
            return Err(())
        }

        if current.is_ascii_digit() {
            let mut state = State::Int;
            let mut float_para: f64 = 10.0;
            let mut value = Number::Int(ascii_to_num(current) as i64);

            index += 1;
            current = chars[index];

            while index < source_len {
                if current.is_ascii_digit() {
                    let num_ascii = ascii_to_num(current);
                    match state {
                        State::Int => {
                            value = value * Number::Int(10);
                            value = value + Number::Int(num_ascii as i64);
                        },
                        State::Float => {
                            value = value + Number::Float((num_ascii as f64) / float_para);
                            float_para *= 10.0;
                        }
                    }
                    index += 1;
                    current = chars[index];
                    continue;
                }
                if current == POINT_ASCII {
                    state = State::Float;
                    value = value.float();
                    index += 1;
                    current = chars[index];
                    continue;
                }
                break;
            }

            if is_num_minus {
                is_num_minus = false;
                value = Number::Int(0) - value;
            }

            last_type = TokenTypes::Number;
            tokens.push(Token::create(
                TokenTypes::Number,
                value,
            ));
            continue;
        }

        if is_identi_ascii(current) {
            let mut value = String::from(current as char);
            last_type = TokenTypes::Identifier;

            index += 1;
            current = chars[index];
            while is_identi_ascii(current) || current.is_ascii_digit() {
                value.push(current as char);
                index += 1;
                current = chars[index];
            }
            tokens.push(Token::create(
                TokenTypes::Identifier,
                value
            ));
            continue;
        }

        // --- --- --- --- --- ---

        const LEFT_PAREN_ASCII  : u8 = 40;
        const RIGHT_PAREN_ASCII : u8 = 41;
        const LEFT_BRACE_ASCII  : u8 = 123;
        const RIGHT_BRACE_ASCII : u8 = 125;

        const PLUS_ASCII     : u8 = 43;
        const MINUS_ASCII    : u8 = 45;
        const MULTIPLY_ASCII : u8 = 42;
        const DIVIDE_ASCII   : u8 = 47;
        const POWER_ASCII    : u8 = 94;
        const EQUAL_ASCII    : u8 = 61;
        
        const SPACE_ASCII  : u8 = 32;
        const RETURN_ASCII : u8 = 13;

        match current {
            LEFT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::create(
                    TokenTypes::Paren, 
                    Symbols::LeftParen
                ));
            },
            RIGHT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::create(
                    TokenTypes::Paren, 
                    Symbols::RightParen
                ));
            },
            LEFT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::create(
                    TokenTypes::Paren, 
                    Symbols::LeftBrace
                ));
            },
            RIGHT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::create(
                    TokenTypes::Paren, 
                    Symbols::RightBrace
                ));
            },

            PLUS_ASCII => {
                if last_type == TokenTypes::Unknown || last_type == TokenTypes::Symbol {
                    is_num_minus = false;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push(Token::create(
                        TokenTypes::Symbol,
                        Symbols::Plus,
                    ));
                }
            },
            MINUS_ASCII => {
                if last_type == TokenTypes::Unknown ||
                   last_type == TokenTypes::Symbol  ||
                   last_type == TokenTypes::Paren 
                {
                    is_num_minus = true;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push(Token::create(
                        TokenTypes::Symbol,
                        Symbols::Minus,
                    ));
                }
            },
            MULTIPLY_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::create(
                    TokenTypes::Symbol,
                    Symbols::Multiply,
                ));
            },
            DIVIDE_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::create(
                    TokenTypes::Symbol,
                    Symbols::Divide,
                ));
            },
            POWER_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::create(
                    TokenTypes::Symbol,
                    Symbols::Power,
                ));
            },
            EQUAL_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::create(
                    TokenTypes::Symbol,
                    Symbols::Equal,
                ));
            },

            SPACE_ASCII  => {},
            RETURN_ASCII => { break; },
            _ => {
                println!("Unknown token: '{}' at index {}.", current as char, index);
                return Err(());
            }
        }

        index += 1;
    }
    Ok(tokens)
}
use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::keywords;

use super::token::{Token, TokenTypes, TokenVec};

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

pub fn tokenize(source: String) -> Result<TokenVec, ()> {
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

        // Number
        if current.is_ascii_digit() {
            last_type = TokenTypes::Number;

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

            let current_token = Token::Number(value);
            tokens.push(current_token);
            continue;
        }

        // Identifier
        if is_identi_ascii(current) {
            let mut value = String::from(current as char);

            index += 1;
            current = chars[index];
            while is_identi_ascii(current) || current.is_ascii_digit() {
                value.push(current as char);
                index += 1;
                current = chars[index];
            }

            // check is keyword
            let mut is_keyword = false;
            let mut current_token: Token;
            let keyword: keywords::Keyword;

            let mut index = 0;
            while index < keywords::KEYWORDS.len() {
                let current = keywords::KEYWORDS[index];
                if value.eq(current) {
                    is_keyword = true;
                    last_type = TokenTypes::Keyword;

                    keyword = keywords::KEYWORDS_ENUM[index];
                    current_token = Token::Keyword(keyword);
                    tokens.push(current_token);
                    break;
                }
                index += 1;
            }

            if !is_keyword {
                last_type = TokenTypes::Identifier;
                current_token = Token::Identi(value);
                tokens.push(current_token);
            }
            continue;
        }

        // --- --- --- --- --- ---

        const LEFT_PAREN_ASCII  : u8 = 40;  // (
        const RIGHT_PAREN_ASCII : u8 = 41;  // )
        const LEFT_BRACE_ASCII  : u8 = 123; // {
        const RIGHT_BRACE_ASCII : u8 = 125; // }

        const PLUS_ASCII     : u8 = 43; // +
        const MINUS_ASCII    : u8 = 45; // -
        const MULTIPLY_ASCII : u8 = 42; // *
        const DIVIDE_ASCII   : u8 = 47; // /
        const POWER_ASCII    : u8 = 94; // ^

        const LESS_THAN_ASCII: u8 = 60; // <
        const MORE_THAN_ASCII: u8 = 62; // >
        const EQUAL_ASCII    : u8 = 61; // =
        
        const SEMICOLON_ASCII  : u8 = 59; // ;  
        const NUMBER_SIGN_ASCII: u8 = 35; // #
        const SPACE_ASCII  : u8 = 32; // ' '
        const TAB_ASCII    : u8 = 9;
        const RETURN_ASCII : u8 = 13; // '\r'

        match current {
            LEFT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::Paren(Symbols::LeftParen));
            },
            RIGHT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::Paren(Symbols::RightParen));
            },
            LEFT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::Paren(Symbols::LeftBrace));
            },
            RIGHT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push(Token::Paren(Symbols::RightBrace));
            },

            PLUS_ASCII => {
                if last_type == TokenTypes::Symbol || last_type == TokenTypes::Unknown {
                    is_num_minus = false;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push(Token::Symbol(Symbols::Plus));
                }
            },
            MINUS_ASCII => {
                if last_type == TokenTypes::Unknown ||
                   last_type == TokenTypes::Symbol
                {
                    is_num_minus = true;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push(Token::Symbol(Symbols::Minus));
                }
            },
            MULTIPLY_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Symbol(Symbols::Multiply));
            },
            DIVIDE_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Symbol(Symbols::Divide));
            },
            POWER_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Symbol(Symbols::Power));
            },
            LESS_THAN_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Symbol(Symbols::LessThan));
            },
            MORE_THAN_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Symbol(Symbols::MoreThan));
            }
            EQUAL_ASCII  => {
                if tokens.len() == 0 {
                    println!("Invalid expression.");
                    return Err(())
                }
                last_type = TokenTypes::Symbol;
                let last_token = tokens.pop().unwrap();

                if let Token::Symbol(last_symbol) = last_token {
                    // if last_symbol
                    if Symbols::is_basic_symbol(last_symbol) {
                        // if last char is: + - * / ^
                        let target_symbol = Symbols::Equal.combine(last_symbol)?;
                        tokens.push(Token::Symbol(target_symbol));
                        index += 1;
                        continue;
                    }
                }

                let current_token = Token::Symbol(Symbols::Equal);
                tokens.push(last_token);
                tokens.push(current_token);
            },

            SEMICOLON_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push(Token::Divider);
            },

            // skip Space and Tab
            SPACE_ASCII => {},
            TAB_ASCII   => {},

            // comment symbol: #
            NUMBER_SIGN_ASCII => break,
            RETURN_ASCII => break,
            _ => {
                println!("Unknown token: '{}' at index {}.", current as char, index);
                return Err(());
            }
        }

        index += 1;
    }
    Ok(tokens)
}
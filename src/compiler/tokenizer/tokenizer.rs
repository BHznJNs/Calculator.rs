use crate::compiler::tokenizer::char_converter::char_converter;
use crate::public::compile_time::parens::Paren;
use crate::public::error::{syntax_error, assignment_error};
use crate::public::value::value::VALUE_TYPE_TUPLES;
use crate::public::value::{number::Number, value::ValueType};
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::keywords;

use super::token::{Token, TokenTypes, TokenVec};

const NUM_ASCII_START: u8 = 48;
const POINT_ASCII    : u8 = 46;
const UNDERLINE_ASCII: u8 = 95;

enum State {
    Int, Float,
}

// identi -> identifier
fn is_identi_ascii(ascii: u8) -> bool {
    return ascii.is_ascii_alphabetic() || ascii == UNDERLINE_ASCII;
}

fn ascii_to_num(ascii: u8) -> u8 {
    return ascii - NUM_ASCII_START
}

pub fn tokenize(source: &String) -> Result<TokenVec, ()> {
    let mut index = 0;

    // is used for check is number minus OR
    // check is in annotation state.
    let mut last_type = TokenTypes::Unknown;

    // use to control if number is minus
    let mut is_num_minus = false;

    let mut tokens = TokenVec::new();
    let chars = source.as_bytes();
    let source_len = source.len();

    while index < source_len {
        let mut current = chars[index];

        if !current.is_ascii() {
            return Err(syntax_error("non-ASCII character")?)
        }

        // Number
        if current.is_ascii_digit() {
            last_type = TokenTypes::Number;

            let mut state = State::Int;
            let mut float_para: f64 = 10.0;
            let mut value =
                Number::Int(ascii_to_num(current) as i64);

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
            tokens.push_back(current_token);
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

            if last_type == TokenTypes::Annotation {
                // Type annotation
                let mut is_valid_type = false;
                let mut value_type: ValueType;

                let mut index = 0;
                while index < VALUE_TYPE_TUPLES.len() {
                    let current = VALUE_TYPE_TUPLES[index];
                    if value.eq(current.0) {
                        is_valid_type = true;
                        value_type = current.1.clone();
                        tokens.push_back(Token::Annotation(value_type))
                    }
                    index += 1;
                }

                if !is_valid_type {
                    let msg = format!("Invalid type: '{}'.", value);
                    return Err(syntax_error(&msg)?)
                }

            } else {
                // check is keyword
                let mut is_keyword = false;
                let keyword: keywords::Keywords;

                let mut index = 0;
                while index < keywords::KEYWORDS.len() {
                    let current = keywords::KEYWORDS[index];
                    if value.eq(current) {
                        is_keyword = true;
                        last_type = TokenTypes::Keywords;

                        keyword = keywords::KEYWORDS_ENUM[index];
                        tokens.push_back(Token::Keywords(keyword));
                        break;
                    }
                    index += 1;
                }

                if !is_keyword {
                    last_type = TokenTypes::Identifier;
                    tokens.push_back(Token::Identi(value));
                }
                
            }
            continue;
        }

        // --- --- --- --- --- ---

        const LEFT_PAREN_ASCII   : u8 = 40;  // (
        const RIGHT_PAREN_ASCII  : u8 = 41;  // )
        const LEFT_BRACKET_ASCII : u8 = 91;  // [
        const RIGHT_BRACKET_ASCII: u8 = 93;  // ]
        const LEFT_BRACE_ASCII   : u8 = 123; // {
        const RIGHT_BRACE_ASCII  : u8 = 125; // }

        const PLUS_ASCII     : u8 = 43; // +
        const MINUS_ASCII    : u8 = 45; // -
        const MULTIPLY_ASCII : u8 = 42; // *
        const DIVIDE_ASCII   : u8 = 47; // /
        const POWER_ASCII    : u8 = 94; // ^

        const NOT_SYMBOL_ASCII : u8 = 33; // !
        const LESS_THAN_ASCII  : u8 = 60; // <
        const MORE_THAN_ASCII  : u8 = 62; // >
        const EQUAL_ASCII      : u8 = 61; // =

        const SINGLE_QUOTE_ASCII : u8 = 39; // '''
        const DOUBLE_QUOTE_ASCII : u8 = 34; // '"'
        const BACKSLASH_ASCII : u8    = 92; // '\'

        const SEMICOLON_ASCII  : u8 = 59; // ;
        const COMMA_ASCII      : u8 = 44; // ,
        const DOLLAR_ASCII     : u8 = 36; // $
        const NUMBER_SIGN_ASCII: u8 = 35; // #
        const SPACE_ASCII      : u8 = 32; // ' '
        const TAB_ASCII        : u8 = 9;  // '\t'
        const NEW_LINE_ASCII   : u8 = 10; // '\n'
        const RETURN_ASCII     : u8 = 13; // '\r'

        match current {
            // Parenthesis
            LEFT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::LeftParen));
            },
            RIGHT_PAREN_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::RightParen));
            },
            LEFT_BRACKET_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::LeftBracket));
            },
            RIGHT_BRACKET_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::RightBracket));
            },
            LEFT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::LeftBrace));
            },
            RIGHT_BRACE_ASCII => {
                last_type = TokenTypes::Paren;
                tokens.push_back(Token::Paren(Paren::RightBrace));
            },

            // Computing symbols
            PLUS_ASCII => {
                if last_type == TokenTypes::Symbol || last_type == TokenTypes::Unknown {
                    is_num_minus = false;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push_back(Token::Symbol(Symbols::Plus));
                }
            },
            MINUS_ASCII => {
                let last_token = tokens.back();
                if last_type  == TokenTypes::Unknown ||
                   last_type  == TokenTypes::Symbol  ||
                   last_token == Some(&Token::Paren(Paren::LeftParen)) ||
                   last_token == Some(&Token::Paren(Paren::LeftBrace)) ||
                   last_token == Some(&Token::Paren(Paren::LeftBracket))
                {
                    is_num_minus = true;
                } else {
                    last_type = TokenTypes::Symbol;
                    tokens.push_back(Token::Symbol(Symbols::Minus));
                }
            },
            MULTIPLY_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::Multiply));
            },
            DIVIDE_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::Divide));
            },
            POWER_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::Power));
            },
            
            NOT_SYMBOL_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::Not));
            },
            LESS_THAN_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::LessThan));
            },
            MORE_THAN_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::MoreThan));
            }
            EQUAL_ASCII => {
                if tokens.len() == 0 {
                    return Err(assignment_error("left-hand value missing")?)
                }

                last_type = TokenTypes::Symbol;
                let last_token = tokens.pop_back().unwrap();

                if let Token::Symbol(last_symbol) = last_token {
                    // if last_symbol
                    if Symbols::is_basic_symbol(last_symbol) {
                        // if last char is: + - * / ^ ! > < = 
                        let target_symbol = Symbols::Equal.combine(last_symbol)?;
                        tokens.push_back(Token::Symbol(target_symbol));
                        index += 1;
                        continue;
                    }
                }

                let current_token = Token::Symbol(Symbols::Equal);
                tokens.push_back(last_token);
                tokens.push_back(current_token);
            },

            // Other symbols
            SINGLE_QUOTE_ASCII | DOUBLE_QUOTE_ASCII => { // String token resolve
                let mut value = String::new();
                let mut is_escape_char = false;
                index += 1;
                current = chars[index];

                while is_escape_char || (current != SINGLE_QUOTE_ASCII && current != DOUBLE_QUOTE_ASCII) {
                    if index == chars.len() - 2 {
                        let msg = format!("Unmatched quote symbol at index {}.", index);
                        return Err(syntax_error(&msg)?)
                    }

                    // if last char is '\', current is
                    // escape character.
                    if is_escape_char {
                        is_escape_char = false;
                        current = char_converter(current)?;
                    } else
                    // when meet '\'
                    if current == BACKSLASH_ASCII {
                        is_escape_char = true;
                        index += 1;
                        current = chars[index];
                        continue;
                    }

                    value.push(current as char);
                    index += 1;
                    current = chars[index];
                }

                index += 1; // skip the right quote.
                tokens.push_back(Token::String(value));
                last_type = TokenTypes::String;
                continue;
            },

            SEMICOLON_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Divider);
            },
            COMMA_ASCII => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Divider);
            },
            DOLLAR_ASCII => {
                // type annotation
                last_type = TokenTypes::Annotation;
            }

            POINT_ASCII  => {
                last_type = TokenTypes::Symbol;
                tokens.push_back(Token::Symbol(Symbols::ObjectReading));
            },

            // skip Space and Tab
            SPACE_ASCII => {},
            TAB_ASCII   => {},

            // comment symbol: #
            // when encount comment symbol,
            // stop resolving current line.
            NUMBER_SIGN_ASCII => break,

            NEW_LINE_ASCII    => break,
            RETURN_ASCII      => break,
            _ => {
                let msg = format!("unknown character '{}' at index {}", current as char, index);
                return Err(syntax_error(&msg)?)
            }
        }

        index += 1;
    }
    Ok(tokens)
}
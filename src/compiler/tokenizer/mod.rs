mod char_converter;
pub mod token;

use std::str::Chars;

use crate::compiler::tokenizer::char_converter::char_converter;
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::keywords::Keyword;
use crate::public::compile_time::parens::Paren;
use crate::public::error::{assignment_error, syntax_error};
use crate::public::value::symbols::Symbols;
use crate::public::value::{number::Number, value::ValueType};
use crate::utils::ascii::{ascii_to_num, is_identi_ascii};

use token::{Token, TokenType, TokenVec};

fn number_resolver(chars: &mut Chars, first_ch: char, index: &mut usize) -> (char, Number) {
    enum State {
        Int,
        Float,
    }

    let mut state = State::Int;
    let mut float_para: f64 = 10.0;
    let mut value = Number::Int(ascii_to_num(first_ch));
    let mut cached_ch = '\0';

    while let Some(ch) = chars.next() {
        *index += 1;

        if ch.is_ascii_digit() {
            let num_ascii = ascii_to_num(ch);
            match state {
                State::Int => {
                    value = value * Number::Int(10);
                    value = value + Number::Int(num_ascii);
                }
                State::Float => {
                    value = value + Number::Float((num_ascii as f64) / float_para);
                    float_para *= 10.0;
                }
            }
            continue;
        }

        if ch == '.' {
            state = State::Float;
            value = value.float();
            continue;
        }
        cached_ch = ch;
        break;
    }
    return (cached_ch, value);
}

fn identi_resolver(chars: &mut Chars, first_ch: char, index: &mut usize) -> (char, String) {
    let mut value = String::from(first_ch);
    let mut cached_ch = '\0';

    while let Some(ch) = chars.next() {
        *index += 1;

        if is_identi_ascii(ch) || ch.is_ascii_digit() {
            value.push(ch);
        } else {
            cached_ch = ch;
            break;
        }
    }
    return (cached_ch, value);
}

pub fn tokenize(source: &String) -> Result<TokenVec, ()> {
    // is used for check is number minus OR
    // check is in annotation state.
    let mut last_type = TokenType::Unknown;

    // use to control if number is minus
    let mut is_num_minus = false;

    let mut tokens = TokenVec::new();

    let mut chars = source.chars();
    let mut cached_ch = '\0';

    let mut index = 0;
    loop {
        let ch = if cached_ch != '\0' {
            cached_ch
        } else if let Some(next_ch) = chars.next() {
            index += 1;
            next_ch
        } else {
            break;
        };
        cached_ch = '\0';

        // Number
        if ch.is_ascii_digit() {
            last_type = TokenType::Number;
            let mut value: Number;
            (cached_ch, value) = number_resolver(&mut chars, ch, &mut index);

            if is_num_minus {
                is_num_minus = false;
                value = Number::Int(0) - value;
            }

            let current_token = Token::Number(value);
            tokens.push_back(current_token);
            continue;
        }
        // Identifier
        if is_identi_ascii(ch) {
            let value: String;
            (cached_ch, value) = identi_resolver(&mut chars, ch, &mut index);

            if last_type == TokenType::Annotation {
                // Type annotation
                match ValueType::is_valid_type(&value) {
                    Some(type__) => {
                        last_type = TokenType::Annotation;
                        tokens.push_back(Token::Annotation(type__));
                    }
                    None => {
                        let msg = format!("Invalid type '{}'", value);
                        return Err(syntax_error(&msg)?);
                    }
                }
            } else {
                // check is keyword
                match Keyword::is_keyword(&value) {
                    Some(keyword) => {
                        last_type = TokenType::Keyword;
                        tokens.push_back(Token::Keyword(keyword));
                    }
                    None => {
                        last_type = TokenType::Identifier;
                        tokens.push_back(Token::Identi(value));
                    }
                }
            }
            continue;
        }

        // --- --- --- --- --- ---

        match ch {
            // Parenthesis
            '(' | ')' | '[' | ']' | '{' | '}' => {
                last_type = TokenType::Paren;
                tokens.push_back(Token::Paren(Paren::from(ch)));
            }

            // Computing symbols
            '+' => {
                if last_type == TokenType::Symbol || last_type == TokenType::Unknown {
                    is_num_minus = false;
                } else {
                    last_type = TokenType::Symbol;
                    tokens.push_back(Token::Symbol(Symbols::Plus));
                }
            }
            '-' => {
                let last_token = tokens.back();
                if last_type == TokenType::Unknown
                    || last_type == TokenType::Symbol
                    || last_token == Some(&Token::Paren(Paren::LeftParen))
                    || last_token == Some(&Token::Paren(Paren::LeftBrace))
                    || last_token == Some(&Token::Paren(Paren::LeftBracket))
                {
                    is_num_minus = true;
                } else {
                    last_type = TokenType::Symbol;
                    tokens.push_back(Token::Symbol(Symbols::Minus));
                }
            }
            '*' | '/' | '%' | '^' | '!' | '<' | '>' | '&' | '|' => {
                last_type = TokenType::Symbol;
                tokens.push_back(Token::Symbol(Symbols::from(ch)));
            }
            '=' => {
                if tokens.len() == 0 {
                    return Err(assignment_error("left-hand value missing")?);
                }

                last_type = TokenType::Symbol;
                let last_token = tokens.pop_back().unwrap();

                if let Token::Symbol(last_symbol) = last_token {
                    // if last_symbol
                    if Symbols::is_basic_symbol(last_symbol) {
                        // if last char is: +  -  *  /  ^  !  >  <  =,
                        // convert it to  : += -= *= /= ^= != >= <= ==.
                        let target_symbol = Symbols::Equal.combine(last_symbol)?;
                        tokens.push_back(Token::Symbol(target_symbol));
                        continue;
                    }
                }

                let current_token = Token::Symbol(Symbols::Equal);
                tokens.push_back(last_token);
                tokens.push_back(current_token);
            }

            // String literal
            '\'' | '\"' => {
                // String token resolve
                let mut value = String::new();
                let mut is_escape_char = false;

                while let Some(mut ch) = chars.next() {
                    index += 1;

                    if is_escape_char || (ch != '\'' && ch != '\"') {
                        // if last char is '\', current is escape character.
                        if is_escape_char {
                            is_escape_char = false;
                            ch = char_converter(ch)?;
                        } else
                        // when meet '\'
                        if ch == '\\' {
                            is_escape_char = true;
                            continue;
                        }
                        value.push(ch);
                    } else {
                        break;
                    }
                }
                tokens.push_back(Token::String(value));
                last_type = TokenType::String;
                continue;
            }

            // Other symbols
            ',' | ';' | ':' => {
                last_type = TokenType::Symbol;
                tokens.push_back(Token::Divider(Divider::from(ch)));
            }
            '$' => {
                // type annotation
                last_type = TokenType::Annotation;
            }

            '.' => {
                last_type = TokenType::Symbol;
                tokens.push_back(Token::Symbol(Symbols::ObjectReading));
            }

            // skip Space and Tab
            ' ' | '\t' => {}

            // comment symbol: # (Number Sign)
            // when encount comment symbol,
            // stop resolving current line.
            '#' => break,
            _ => {
                let msg = format!("unknown character '{}' at index {}", ch, index);
                return Err(syntax_error(&msg)?);
            }
        }
    }
    return Ok(tokens);
}

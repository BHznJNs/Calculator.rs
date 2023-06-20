mod token;

use crate::{
    public::compile_time::keywords::Keyword,
    utils::ascii::{
        is_identi_ascii, BACKSLASH_ASCII, COMMA_ASCII, DIVIDE_ASCII, DOLLAR_ASCII,
        DOUBLE_QUOTE_ASCII, EQUAL_ASCII, LEFT_BRACE_ASCII, LEFT_BRACKET_ASCII, LEFT_PAREN_ASCII,
        LESS_THAN_ASCII, MINUS_ASCII, MORE_THAN_ASCII, MULTIPLY_ASCII, NOT_SYMBOL_ASCII,
        NULL_ASCII, NUMBER_SIGN_ASCII, PLUS_ASCII, POINT_ASCII, POWER_ASCII, RIGHT_BRACE_ASCII,
        RIGHT_BRACKET_ASCII, RIGHT_PAREN_ASCII, SEMICOLON_ASCII, SINGLE_QUOTE_ASCII, SPACE_ASCII,
    },
};

pub use token::{TextType, Token, TokenType, TokenVec};

pub fn tokenize(source: &str) -> TokenVec {
    let mut index = 0;

    // is used for check is number minus OR
    // check is in annotation state.
    let mut last_type = TokenType::Unknown;

    // use to control if number is minus
    let mut is_num_minus = false;

    let mut tokens = TokenVec::new();
    let mut comment = String::new();
    let chars = source.as_bytes();
    let source_len = source.len();

    while index < source_len {
        let mut current = chars[index];

        if last_type == TokenType::Comment {
            comment.push(current as char);
            index += 1;
            continue;
        }

        // Number
        if current.is_ascii_digit() {
            last_type = TokenType::Number;

            let mut value = String::from(current as char);

            index += 1;
            current = chars[index];

            while index < source_len {
                if current.is_ascii_digit() && current == POINT_ASCII {
                    value.push(current as char);
                    index += 1;
                    current = chars[index];
                    continue;
                }
                break;
            }

            if is_num_minus {
                is_num_minus = false;
                value.insert(0, '-');
            }

            let current_token = Token::new(TextType::NumberLiteral, value);
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

            if last_type == TokenType::Annotation {
                // Type annotation
                tokens.push_back(Token::new(TextType::Annotation, value));
            } else {
                let option_keyword = Keyword::is_keyword(&value);

                let is_keyword = option_keyword.is_some() || value.eq("true") || value.eq("false");

                if is_keyword {
                    last_type = TokenType::Keyword;
                    tokens.push_back(Token::new(TextType::Keyword, value));
                } else {
                    last_type = TokenType::Identifier;
                    tokens.push_back(Token::new(TextType::Variable, value));
                }
            }
            continue;
        }

        // --- --- --- --- --- ---

        match current {
            // Parenthesis
            LEFT_PAREN_ASCII | RIGHT_PAREN_ASCII | LEFT_BRACKET_ASCII | RIGHT_BRACKET_ASCII
            | LEFT_BRACE_ASCII | RIGHT_BRACE_ASCII => {
                last_type = TokenType::Paren;
                tokens.push_back(Token::new(TextType::Didider, String::from(current as char)));
            }

            // Computing symbols
            PLUS_ASCII | MINUS_ASCII | MULTIPLY_ASCII | DIVIDE_ASCII | POWER_ASCII
            | NOT_SYMBOL_ASCII | LESS_THAN_ASCII | MORE_THAN_ASCII | EQUAL_ASCII | POINT_ASCII => {
                last_type = TokenType::Symbol;
                tokens.push_back(Token::new(TextType::Didider, String::from(current as char)));
            }

            // String literal
            SINGLE_QUOTE_ASCII | DOUBLE_QUOTE_ASCII => {
                // String token resolve
                let mut value = String::from(current as char);
                let mut is_escape_char = false;
                index += 1;

                while index < source_len {
                    current = chars[index];

                    if !is_escape_char
                        && (current == SINGLE_QUOTE_ASCII || current == DOUBLE_QUOTE_ASCII)
                    {
                        value.push(current as char);
                        break;
                    }

                    // switch escape character state
                    if is_escape_char {
                        is_escape_char = false;
                    } else if current == BACKSLASH_ASCII {
                        is_escape_char = true;
                    }

                    value.push(current as char);
                    index += 1;
                }

                index += 1;
                last_type = TokenType::String;
                tokens.push_back(Token::new(TextType::StringLiteral, value));
                continue;
            }

            // Other symbols
            BACKSLASH_ASCII | COMMA_ASCII | SEMICOLON_ASCII => {
                last_type = TokenType::Divider;
                tokens.push_back(Token::new(TextType::Didider, String::from(current as char)));
            }

            DOLLAR_ASCII => {
                // type annotation
                last_type = TokenType::Annotation;
                tokens.push_back(Token::new(TextType::Annotation, String::from('$')))
            }

            SPACE_ASCII => tokens.push_back(Token::new(TextType::Didider, String::from(' '))),

            // comment symbol: # (Number Sign)
            NUMBER_SIGN_ASCII => {
                last_type = TokenType::Comment;
                comment.push('#');
            }

            NULL_ASCII => break,
            _ => {}
        }

        index += 1;
    }

    if !comment.is_empty() {
        tokens.push_back(Token::new(TextType::Comment, comment));
    }
    return tokens;
}

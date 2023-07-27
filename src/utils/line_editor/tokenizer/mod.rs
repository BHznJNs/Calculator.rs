mod token;

use crate::{public::compile_time::keywords::Keyword, utils::ascii::is_identi_ascii};

pub use token::{TextType, Token, TokenType, TokenVec};

pub fn tokenize(source: &str) -> TokenVec {
    // is used for check is number minus OR
    // check whether is in annotation state.
    let mut last_type = TokenType::Unknown;

    // use to control if number is minus
    let mut is_num_minus = false;

    let mut tokens = TokenVec::new();
    let mut comment = String::new();

    let mut chars = source.chars();
    let mut cached_ch = '\0';
    loop {
        let ch = if cached_ch != '\0' {
            cached_ch
        } else if let Some(next_ch) = chars.next() {
            next_ch
        } else {
            break;
        };
        cached_ch = '\0';

        if last_type == TokenType::Comment {
            comment.push(ch);
            continue;
        }

        // Number
        if ch.is_ascii_digit() {
            last_type = TokenType::Number;

            let mut value = String::from(ch);
            if is_num_minus {
                is_num_minus = false;
                value.insert(0, '-');
            }

            while let Some(ch) = chars.next() {
                if ch.is_ascii_digit() || ch == '.' {
                    value.push(ch);
                } else {
                    cached_ch = ch;
                    break;
                }
            }

            let current_token = Token::new(TextType::NumberLiteral, value);
            tokens.push(current_token);
            continue;
        }

        // Identifier
        if is_identi_ascii(ch) {
            let mut value = String::from(ch);

            while let Some(ch) = chars.next() {
                if is_identi_ascii(ch) || ch.is_ascii_digit() {
                    value.push(ch)
                } else {
                    cached_ch = ch;
                    break;
                }
            }

            if last_type == TokenType::Annotation {
                // Type annotation
                tokens.push(Token::new(TextType::Annotation, value));
            } else {
                let option_keyword = Keyword::is_keyword(&value);
                let is_keyword = option_keyword.is_some() || value.eq("true") || value.eq("false");

                if is_keyword {
                    last_type = TokenType::Keyword;
                    tokens.push(Token::new(TextType::Keyword, value));
                } else {
                    last_type = TokenType::Identifier;
                    tokens.push(Token::new(TextType::Variable, value));
                }
            }
            continue;
        }

        // --- --- --- --- --- ---

        match ch {
            // Parenthesis
            '(' | ')' | '[' | ']' | '{' | '}' => {
                last_type = TokenType::Paren;
                tokens.push(Token::new(TextType::Paren, String::from(ch)));
            }
            // Computing symbols
            '+' | '-' | '*' | '/' | '^' | '!' | '<' | '>' | '=' | '.' | '&' | '|' => {
                last_type = TokenType::Symbol;
                tokens.push(Token::new(TextType::Symbol, String::from(ch)));
            }
            // String literal
            '\'' | '\"' => {
                // String token resolve
                let mut value = String::from(ch);
                let mut is_escape_char = false;

                while let Some(ch) = chars.next() {
                    if !is_escape_char && (ch == '\'' || ch == '\"') {
                        value.push(ch);
                        break;
                    }

                    // switch escape character state
                    if is_escape_char {
                        is_escape_char = false;
                    } else if ch == '\\' {
                        is_escape_char = true;
                    }

                    value.push(ch);
                }

                last_type = TokenType::String;
                let current_token = Token::new(TextType::StringLiteral, value);
                tokens.push(current_token);
                continue;
            }
            // Other symbols
            '\\' | ',' | ';' => {
                last_type = TokenType::Divider;
                tokens.push(Token::new(TextType::Didider, String::from(ch)));
            }

            '$' => {
                // type annotation
                last_type = TokenType::Annotation;
                tokens.push(Token::new(TextType::Annotation, String::from('$')))
            }

            ' ' => tokens.push(Token::new(TextType::Didider, String::from(' '))),

            // comment symbol: # (Number Sign)
            '#' => {
                last_type = TokenType::Comment;
                comment.push('#');
            }
            _ => {
                // is not allowed character
                last_type = TokenType::Unknown;
                tokens.push(Token::new(TextType::Unknown, String::from(ch)));
            }
        }
    }
    if !comment.is_empty() {
        tokens.push(Token::new(TextType::Comment, comment));
    }
    return tokens;
}

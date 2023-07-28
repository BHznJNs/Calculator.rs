use crate::compiler::analyzer::resolvers::sequence;
use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTVec;
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;

pub fn resolve(tokens: &mut TokenVec) -> Result<ASTVec, ()> {
    // statement body sequence resolve
    // without LeftBrace
    // template: `{ ...; ... }`

    #[derive(PartialEq)]
    enum State {
        Inner,
        Outer,
    }

    let first_index = 0;
    let mut state = State::Outer;

    // for all type of paren: Paren | Brace | Bracket
    let mut paren_count = 1;
    let mut sub_tokens = TokenVec::new();
    let mut result_params = ASTVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();

        let is_divider = current == Token::Divider(Divider::Semicolon);
        let is_left_paren = current == Token::Paren(Paren::LeftBrace)
            || current == Token::Paren(Paren::LeftParen)
            || current == Token::Paren(Paren::LeftBracket);
        let is_right_paren = current == Token::Paren(Paren::RightBrace)
            || current == Token::Paren(Paren::RightParen)
            || current == Token::Paren(Paren::RightBracket);

        if is_left_paren {
            state = State::Inner;
            paren_count += 1;
        }
        if is_divider && (state == State::Outer) {
            let sub_sequence_node = sequence::resolve(&mut sub_tokens)?;
            sub_tokens.clear();
            result_params.push(sub_sequence_node);
            continue;
        }
        if is_right_paren {
            paren_count -= 1;
            if paren_count == 1 {
                state = State::Outer;
            }
            if paren_count == 0 {
                let sub_sequence_node = sequence::resolve(&mut sub_tokens)?;
                sub_tokens.clear();
                result_params.push(sub_sequence_node);
                break;
            }
        }

        sub_tokens.push_back(current);
    }
    Ok(result_params)
}

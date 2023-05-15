use crate::compiler::analyzer::resolvers::sequence;
use crate::compiler::tokenizer::token::{TokenVec, Token};
use crate::public::compile_time::ast::ast_enum::ASTVec;
use crate::public::value::parens::Parens;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ASTVec, ()> {
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

        let is_divider =
            current == Token::Divider;
        let is_left_paren =
            current == Token::Paren(Parens::LeftBrace) ||
            current == Token::Paren(Parens::LeftParen) ||
            current == Token::Paren(Parens::LeftBracket);
        let is_right_paren =
            current == Token::Paren(Parens::RightBrace) ||
            current == Token::Paren(Parens::RightParen) ||
            current == Token::Paren(Parens::RightBracket);

        if is_left_paren {
            state = State::Inner;
            paren_count += 1;
        }
        if is_divider && (state == State::Outer) {
            let sub_sequence_node =
                sequence::resolve(&mut sub_tokens)?;
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
                let sub_sequence_node =
                    sequence::resolve(&mut sub_tokens)?;
                sub_tokens.clear();
                result_params.push(sub_sequence_node);
                break;
            }
        }

        sub_tokens.push_back(current);
    }
    Ok(result_params)
}
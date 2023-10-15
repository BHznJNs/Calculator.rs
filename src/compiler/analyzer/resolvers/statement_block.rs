use crate::compiler::analyzer::resolvers::sequence;
use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTVec;
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;
use crate::public::error::CalcResult;

pub fn resolve(tokens: &mut TokenVec) -> CalcResult<ASTVec> {
    // statement body sequence resolve
    // without LeftBrace
    // template: `{ ...; ... }`

    #[derive(PartialEq)]
    enum State {
        Inner,
        Outer,
    }

    let mut state = State::Outer;

    // for all type of paren: Paren | Brace | Bracket
    let mut paren_count = 1;
    let mut sub_tokens = TokenVec::new();
    let mut result_params = ASTVec::new();

    while let Some(token) = tokens.pop_front() {
        let is_divider = token == Token::Divider(Divider::Semicolon);
        let is_left_paren = token == Token::Paren(Paren::LeftBrace)
            || token == Token::Paren(Paren::LeftParen)
            || token == Token::Paren(Paren::LeftBracket);
        let is_right_paren = token == Token::Paren(Paren::RightBrace)
            || token == Token::Paren(Paren::RightParen)
            || token == Token::Paren(Paren::RightBracket);

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

        sub_tokens.push_back(token);
    }
    return Ok(result_params);
}

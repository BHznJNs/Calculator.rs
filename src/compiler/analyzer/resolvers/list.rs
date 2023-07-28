use crate::compiler::analyzer::resolvers::expression;
use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;

pub fn resolve(tokens: &mut TokenVec, identi_paren: Paren) -> Result<Vec<ExpressionNode>, ()> {
    // examples:
    // 1, 2)
    // a, 1)
    // 1, [2, 3])
    // 1, {a + 1})

    #[derive(PartialEq)]
    enum State {
        Inner,
        Outer,
    }

    fn element_resolve(
        sub_tokens: &mut TokenVec,
        elements: &mut Vec<ExpressionNode>,
    ) -> Result<(), ()> {
        if sub_tokens.len() > 0 {
            let element = expression::resolve(sub_tokens)?;
            sub_tokens.clear();
            elements.push(element);
        }
        Ok(())
    }

    let mut state = State::Outer;
    let mut paren_count = 1;
    let mut elements = Vec::<ExpressionNode>::new();
    let mut sub_tokens = TokenVec::new();

    loop {
        if tokens.len() == 0 {
            return Err(syntax_error("Unmatched parentheses")?);
        }

        let current = tokens.pop_front().unwrap();

        let is_divider = current == Token::Divider(Divider::Comma);
        let is_identi_paren = current == Token::Paren(identi_paren);
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
            element_resolve(&mut sub_tokens, &mut elements)?;
            continue;
        }
        if is_right_paren {
            paren_count -= 1;
            if paren_count == 1 {
                state = State::Outer;
            }

            if is_identi_paren && paren_count == 0 {
                element_resolve(&mut sub_tokens, &mut elements)?;
                break;
            }
        }

        sub_tokens.push_back(current);
    }

    Ok(elements)
}

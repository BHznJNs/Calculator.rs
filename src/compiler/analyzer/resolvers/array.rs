use crate::public::value::parens::Parens;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::expression;

#[derive(PartialEq)]
enum State {
    Inner,
    Outer,
}

pub fn literal_resolve(
    tokens: &mut TokenVec
) -> Result<ASTNode, ()> {
    fn element_resolve(
        sub_tokens: &mut TokenVec,
        elements: &mut ASTNodeVec,
    ) -> Result<(), ()> {
        if sub_tokens.len() > 0 {
            let element_params =
                expression::resolve(sub_tokens, false)?;
            sub_tokens.clear();
            elements.push(ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(element_params)
            });
        }
        Ok(())
    }

    let first_index = 0;
    let mut state = State::Outer;
    let mut paren_count = 1;
    let mut elements = ASTNodeVec::new();
    let mut sub_tokens = TokenVec::new();

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
            element_resolve(&mut sub_tokens, &mut elements)?;
            continue;
        }
        if is_right_paren {
            paren_count -= 1;
            if paren_count == 1 {
                state = State::Outer;
            }
            if paren_count == 0 {
                element_resolve(&mut sub_tokens, &mut elements)?;
                break;
            }
        }

        sub_tokens.push_back(current);
    }

    let array_node = ASTNode {
        type__: ASTNodeTypes::ArrayLiteral,
        params: Some(elements),
    };

    Ok(array_node)
}

pub fn reading_resolve(
    arr_name: String,
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    // example:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`
    let first_index = 0;
    let mut bracket_count = 1;
    let mut params = ASTNodeVec::new();
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Parens::LeftBracket) {
            bracket_count += 1;
        }
        if current == Token::Paren(Parens::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }
    let reading_index_expression_nodes =
        expression::resolve(&mut sub_tokens, false)?;
    params.push(ASTNode {
        type__: ASTNodeTypes::Expression,
        params: Some(reading_index_expression_nodes),
    });

    // sub array element reading
    let next_token = tokens.pop_front();
    if let Some(Token::Paren(Parens::LeftBracket)) = next_token {
        let sub_element_reading =
            reading_resolve(arr_name.clone(), tokens)?;
        params.push(sub_element_reading);
    } else if next_token == None {
        // empty
    } else {
        // exist next_token and next_token
        // is not equal Token::Paren(Parens::LeftBracket)
        tokens.push_front(next_token.unwrap())
    }

    let current_node = ASTNode {
        type__: ASTNodeTypes::ArrayElementReading(arr_name),
        params: Some(params)
    };
    Ok(current_node)
}
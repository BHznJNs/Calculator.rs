use crate::public::value::parens::Parens;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::compile_time::keywords::Keywords;
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::expression;
use super::sequence;

#[derive(PartialEq)]
enum State {
    Inner,
    Outer,
}

// get condition for statement
fn statement_condition_resolve(
    tokens: &mut TokenVec,
    params: &mut ASTNodeVec,
) -> Result<(), ()> {
    let first_index = 0;
    let mut sub_tokens = TokenVec::new();
    // sub condition tokens

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();
        //                         '{'
        if current == Token::Paren(Parens::LeftBrace) { break }
        sub_tokens.push_back(current);
    }
    let condition =
        expression::resolve(&mut sub_tokens, false)?;
    params.push(ASTNode {
        type__: ASTNodeTypes::Expression,
        params: Some(condition),
    });
    Ok(())
}
pub fn statement_body_resolve(
    tokens: &mut TokenVec,
    params: &mut ASTNodeVec,
) -> Result<(), ()> {
    // statement body sequence resolve
    // without LeftBrace
    // template: `{ ...; ... }`

    let first_index = 0;
    let mut state = State::Outer;
    let mut paren_count = 1; // All type of paren: Paren | Brace | Bracket
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
            let sub_sequence_node =
                sequence::resolve(&mut sub_tokens)?;
            sub_tokens.clear();
            params.push(sub_sequence_node);
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
                params.push(sub_sequence_node);
                break;
            }
        }

        sub_tokens.push_back(current);
    }
    Ok(())
}

pub fn resolve(
    keyword: Keywords,
    tokens: &mut TokenVec
) -> Result<ASTNodeVec, ()> {
    // remove the keyword token
    tokens.pop_front();

    let mut params = ASTNodeVec::new();

    match keyword {
        Keywords::Out => {
            let output_expression =
                expression::resolve(tokens, false)?;
            let current_node = ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(output_expression),
            };
            params.push(current_node);
        },
        Keywords::For => {
            statement_condition_resolve(tokens, &mut params)?;
            statement_body_resolve(tokens, &mut params)?;
        },
        Keywords::If => {
            statement_condition_resolve(tokens, &mut params)?;
            statement_body_resolve(tokens, &mut params)?;
        },
        Keywords::Import => {
            if tokens.len() == 0 {
                println!("Invalid import statement: module name missing.");
                return Err(())
            }
            let next_token = tokens.pop_front().unwrap();
            let Token::Identi(module_name) = next_token else {
                println!("Invalid import statement: invalid module name.");
                return Err(())
            };

            params.push(ASTNode {
                type__: ASTNodeTypes::Variable(module_name),
                params: None,
            });
        },

        Keywords::Break => {
            let return_expression =
                expression::resolve(tokens, false)?;
            let current_node = ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(return_expression),
            };
            params.push(current_node);
        },
        Keywords::Continue => {}, // Do nothing
        _ => {
            println!("Tokenizer error: unexpected keyword '{}'.", keyword);
            return Err(())
        }
    }

    Ok(params)
}
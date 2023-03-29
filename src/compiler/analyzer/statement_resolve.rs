use crate::public::value::parens::Parens;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::compile_time::keywords::Keywords;
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::expression_resolve;
use super::sequence_resolve;

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
        expression_resolve::resolve(&mut sub_tokens, false)?;
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
    let mut brace_count = 1;
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();

        let is_divider =
            current == Token::Divider;
        let is_left_brace =
            current == Token::Paren(Parens::LeftBrace);
        let is_right_brace =
            current == Token::Paren(Parens::RightBrace);

        if is_left_brace {
            brace_count += 1;
        }
        if is_divider {
            let sub_sequence_node =
                sequence_resolve::resolve(&mut sub_tokens)?;
            sub_tokens.clear();
            params.push(sub_sequence_node);
            continue;
        }
        if is_right_brace {
            brace_count -= 1;
            if brace_count == 0 {
                let sub_sequence_node =
                    sequence_resolve::resolve(&mut sub_tokens)?;
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
                expression_resolve::resolve(tokens, false)?;
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
        Keywords::Continue => {},
        Keywords::Break => {},
        _ => {
            println!("Tokenizer error: unexpected function definition.");
            return Err(())
        }
    }

    Ok(params)
}
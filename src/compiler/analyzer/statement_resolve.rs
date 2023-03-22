use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::compile_time::keywords::Keyword;
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::expression_resolve::expression_resolve;
use super::sequence_resolve::sequence_resolve;

fn statement_body_resolve(
    tokens: &mut TokenVec,
    params: &mut ASTNodeVec,
) -> Result<(), ()> {
    let first_index = 0;

    // get condition for statement
    let mut sub_tokens = TokenVec::new();
    // sub condition tokens
    while first_index < tokens.len() {
        let current = tokens.remove(first_index);

        if current == Token::Paren(Parens::LeftBrace) {break}
        sub_tokens.push(current);
    }
    let condition =
        expression_resolve(&mut sub_tokens, false)?;
    params.push(ASTNode {
        type__: ASTNodeTypes::Expression,
        params: Some(condition),
    });

    // --- --- --- --- --- ---

    // statement body sequence resolve
    // within `{ ... }`
    let mut sub_tokens = TokenVec::new();
    while first_index < tokens.len() {
        let current = tokens.remove(first_index);

        let is_divider = current == Token::Divider;
        let is_rightbrace = current == Token::Paren(Parens::RightBrace);

        if is_divider || is_rightbrace {
            // when `;` OR `}`
            let sub_sequence_node =
                sequence_resolve(&mut sub_tokens)?;
            sub_tokens.clear();
            params.push(sub_sequence_node);

            if is_rightbrace { break }
        } else {
            sub_tokens.push(current);
        }
    }
    Ok(())
}

pub fn statement_resolve(
    keyword: Keyword,
    tokens: &mut TokenVec
) -> Result<ASTNodeVec, ()> {
    // remove the keyword token
    let first_index = 0;
    tokens.remove(first_index);

    let mut params = ASTNodeVec::new();

    match keyword {
        Keyword::Out => {
            let output_expression =
                expression_resolve(tokens, false)?;
            let current_node = ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(output_expression),
            };
            params.push(current_node);
        },
        Keyword::For => {
            statement_body_resolve(tokens, &mut params)?;
        },
        Keyword::If => {
            statement_body_resolve(tokens, &mut params)?;
        },
        Keyword::Continue => params.push(ASTNode {
            type__: ASTNodeTypes::Statement(Keyword::Continue),
            params: None,
        }),
        Keyword::Break => params.push(ASTNode {
            type__: ASTNodeTypes::Statement(Keyword::Continue),
            params: None,
        }),
    }

    Ok(params)
}
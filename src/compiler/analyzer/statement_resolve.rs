use crate::public::symbols::Symbols;
use crate::public::token::{Token, TokenVec};
use crate::public::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::keywords::Keyword;

use super::expression_resolve::expression_resolve;
use super::sequence_resolve::sequence_resolve;

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
        Keyword::Loop => {
            // get times of loop
            let mut sub_tokens = TokenVec::new();
            while first_index < tokens.len() {
                let current = tokens.remove(first_index);

                if current == Token::Paren(Symbols::LeftBrace) {break}
                sub_tokens.push(current);
            }
            let loop_count_expression =
                expression_resolve(&mut sub_tokens, false)?;
            params.push(ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(loop_count_expression),
            });

            // --- --- --- --- --- ---

            let mut sub_tokens = TokenVec::new();
            while first_index < tokens.len() {
                let current = tokens.remove(first_index);

                let is_divider = current == Token::Divider;
                let is_rightbrace = current == Token::Paren(Symbols::RightBrace);

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
        },
    }

    Ok(params)
}
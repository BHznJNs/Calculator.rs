use crate::compiler::tokenizer::token::TokenVec;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::compiler::tokenizer::token::Token;


use super::statement_resolve::statement_resolve;
use super::expression_resolve::expression_resolve;

pub fn sequence_resolve(
    tokens: &mut TokenVec
) -> Result<ASTNode, ()> {
    if tokens.len() == 0 {
        // blank line || line comment
        Ok(ASTNode {
            type__: ASTNodeTypes::Comment,
            params: None,
        })

    } else
    if let Token::Keyword(keyword) = tokens[0] {
        // regard the whole sequence as a statement
        let statement_nodes =
            statement_resolve(keyword, tokens)?;

        let current_node = ASTNode {
            type__: ASTNodeTypes::Statement(keyword),
            params: Some(statement_nodes),
        };
        Ok(current_node)
    } else {
        // regard the whole sequence as a expression
        let expression_nodes =
            expression_resolve(tokens, false)?;

        let current_node = ASTNode {
            type__: ASTNodeTypes::Expression,
            params: Some(expression_nodes),
        };

        Ok(current_node)
    }
}
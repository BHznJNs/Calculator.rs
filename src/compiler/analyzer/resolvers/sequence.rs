use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;

use super::{expression, statement};

pub fn resolve(tokens: &mut TokenVec) -> Result<ASTNode, ()> {
    if tokens.len() == 0 {
        // blank line || line comment
        Ok(ASTNode::Comment)
    } else if let Token::Keywords(keyword) = tokens[0] {
        // if matches keyword,
        // regard the whole sequence as a statement
        let statement_nodes = statement::resolve(keyword, tokens)?;
        Ok(ASTNode::Statement(statement_nodes.into()))
    } else {
        // regard the whole sequence as an expression
        let expression_nodes = expression::resolve(tokens)?;
        Ok(ASTNode::Expression(expression_nodes.into()))
    }
}

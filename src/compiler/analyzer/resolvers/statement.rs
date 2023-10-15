use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::{
    ExpressionNode, ForStatement, IfStatement, ImportNode, ModuleType,
};
use crate::public::compile_time::parens::Paren;
use crate::public::compile_time::{ast::types::StatementNode, keywords::Keyword};
use crate::public::error::{import_error, syntax_error, CalcResult};

use super::{expression, statement_block};

fn statement_condition_resolve(tokens: &mut TokenVec) -> CalcResult<ExpressionNode> {
    let mut sub_tokens = TokenVec::new(); // sub condition tokens

    while let Some(token) = tokens.pop_front() {
        if token == Token::Paren(Paren::LeftBrace) {
            break;
        }
        sub_tokens.push_back(token);
    }
    let expression_node = expression::resolve(&mut sub_tokens)?;
    return Ok(expression_node);
}

pub fn resolve(keyword: Keyword, tokens: &mut TokenVec) -> CalcResult<StatementNode> {
    // remove the keyword token
    tokens.pop_front();

    let result = match keyword {
        Keyword::Out => {
            let output_expression = expression::resolve(tokens)?;
            StatementNode::Output(output_expression)
        }
        Keyword::For => StatementNode::ForLoop(ForStatement {
            loop_count: statement_condition_resolve(tokens)?,
            body: statement_block::resolve(tokens)?,
        }),
        Keyword::If => StatementNode::Condition(IfStatement {
            condition: statement_condition_resolve(tokens)?,
            body: statement_block::resolve(tokens)?,
        }),

        Keyword::Import => {
            let Some(next_token) = tokens.pop_front() else {
                return Err(syntax_error("module name expected"));
            };
            let Token::Identi(module_name) = next_token else {
                return Err(import_error("invalid module name"));
            };
            let node = ImportNode {
                type__: ModuleType::BuildIn,
                target: module_name,
            };
            StatementNode::Import(node)
        }

        Keyword::Global => {
            let mut sub_expression = expression::resolve(tokens)?;
            let first_node = sub_expression.elements.remove(0);
            if let ASTNode::Assignment(sub_node) = first_node {
                StatementNode::GlobalAssignment(*sub_node)
            } else {
                return Err(syntax_error(
                    "assignment expression is expected following the keyword `glo`",
                ));
            }
        }

        Keyword::Break => StatementNode::Break(expression::resolve(tokens)?),
        Keyword::Continue => StatementNode::Continue, // Do nothing
        _ => {
            // example:
            // if 1 {new}
            let msg = format!("unexpected keyword '{}' at start of statement", keyword);
            return Err(syntax_error(&msg));
        }
    };
    return Ok(result);
}

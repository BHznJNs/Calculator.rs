use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::ExpressionNode;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;
use crate::public::value::symbols::Symbols;

use super::super::array;
use super::{assignment, invocation, object_reading};

pub fn resolve(var_node: ASTNode, tokens: &mut TokenVec) -> Result<ASTNode, ()> {
    let is_more_token = tokens.len() > 0;

    let result_node = if is_more_token {
        let next_token = tokens.pop_front().unwrap();
        // let var_node_rc = Rc::new(var_node.clone());

        let current_node = match next_token {
            Token::Paren(Paren::LeftParen) => {
                // invocation for:
                // build-in function || lazy-expression ||
                // user-defined-function || object method.

                let mut invoke_node = invocation::resolve(var_node.clone(), tokens)?;

                // if object-method, insert object variable as `self` param
                if let ASTNode::ObjectReading(node) = var_node {
                    let obj_clone = node.obj_node.clone();
                    let inserted_expression = ExpressionNode {
                        elements: vec![obj_clone],
                    };
                    invoke_node.params.insert(0, inserted_expression);
                }
                ASTNode::Invocation(invoke_node.into())
            }
            Token::Paren(Paren::LeftBracket) => {
                // array element reading
                let reading_node = array::reading_resolve(var_node, tokens)?;
                ASTNode::ArrayElementReading(reading_node.into())
            }
            Token::Paren(_) => {
                // examples:
                // ...  PI)
                // ..., PI]
                tokens.push_front(next_token);
                return Ok(var_node);
            }

            Token::Symbol(Symbols::ObjectReading) => {
                // object property / method reading
                let current_node = object_reading::resolve(var_node, tokens)?.into();
                ASTNode::ObjectReading(current_node)
            }
            Token::Symbol(symbol) => {
                if Symbols::is_equal_symbol(symbol) {
                    // assignment
                    let current_node = assignment::resolve(tokens, symbol, var_node)?.into();
                    ASTNode::Assignment(current_node)
                } else {
                    // next_token is symbol: + - * /
                    tokens.push_front(next_token);
                    return Ok(var_node);
                }
            }
            _ => {
                let msg = format!("unexpected token `{}`", next_token);
                return Err(syntax_error(&msg)?);
            }
        };
        resolve(current_node, tokens)?
    } else {
        var_node
    };

    Ok(result_node)
}

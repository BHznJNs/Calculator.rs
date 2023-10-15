use std::rc::Rc;

use crate::compiler::tokenizer::token::{Token, TokenVec};
use crate::public::compile_time::ast::types::{ClassDefinitionNode, FunctionDefinitionNode};
use crate::public::compile_time::dividers::Divider;
use crate::public::compile_time::parens::Paren;
use crate::public::error::{syntax_error, CalcResult};
use crate::public::value::function::UserDefinedFnParam;
use crate::public::value::oop::class::Property;
use crate::public::value::symbols::Symbols;
use crate::public::value::ValueType;

use super::function_definition;

pub fn resolve(tokens: &mut TokenVec) -> CalcResult<ClassDefinitionNode> {
    // no `cl` keyword
    // example:
    // { prop $_, method=(self $_){do something...} }

    if tokens.is_empty() {
        return Err(syntax_error("missing class body"));
    }

    let mut properties = Vec::<Property>::new();
    let mut method_nodes = Vec::<Rc<FunctionDefinitionNode>>::new();

    let first_token = tokens.pop_front().unwrap();

    if first_token == Token::Paren(Paren::LeftBrace) {
        loop {
            if tokens.is_empty() {
                return Err(syntax_error("unmatched brace"));
            }

            let current = tokens.pop_front().unwrap();

            if let Token::Identi(identi) = current {
                let Some(next_token) = tokens.pop_front() else {
                    // if no token follows the property
                    return Err(syntax_error("unmatched brace"));
                };

                match next_token {
                    Token::Annotation(type__) => properties.push(Property(type__, identi)),
                    Token::Symbol(Symbols::Equal) => {
                        // current as class method
                        let mut method_node = function_definition::resolve(tokens)?;
                        method_node.params.insert(
                            0,
                            UserDefinedFnParam {
                                type__: ValueType::Object,
                                identi: String::from("self"),
                            },
                        );
                        method_node.name = Some(identi);
                        method_nodes.push(method_node.into())
                    }
                    _ => {
                        let msg = format!("unexpected token {} in class body", next_token);
                        return Err(syntax_error(&msg));
                    }
                }
            } else if current == Token::Divider(Divider::Semicolon) {
                continue;
            } else if current == Token::Paren(Paren::RightBrace) {
                break;
            } else {
                let msg = format!("unexpected token {} in class body", current);
                return Err(syntax_error(&msg));
            }
        }
    } else {
        return Err(syntax_error("expected class-definition body"));
    }
    return Ok(ClassDefinitionNode {
        properties,
        method_nodes,
    });
}

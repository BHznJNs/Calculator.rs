use crate::compiler::analyzer::resolvers::compose::compose;
use crate::compiler::analyzer::resolvers::{function_definition, class_definition, instantiation};
use crate::public::compile_time::ast::types::{ExpressionNode, VariableNode};
use crate::public::compile_time::keywords::Keyword;
use crate::public::compile_time::parens::Paren;
use crate::public::error::syntax_error;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::ast_enum::{ASTNode, ASTVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::{lazy_expression, array};
use super::symbol_priority::compare;

pub fn resolve(
    tokens: &mut TokenVec,
) -> Result<ExpressionNode, ()> {
    let mut params = ASTVec::new();
    let first_index = 0;

    while first_index < tokens.len() {
        let current =
            tokens.pop_front().unwrap();
        
        match current {
            Token::Number(num) =>
                params.push(ASTNode::NumberLiteral(num)),
            Token::String(str) =>
                params.push(ASTNode::StringLiteral(str)),
            Token::Symbol(sym) => {
                if sym == Symbols::Equal {
                    return Err(syntax_error("invalid left-hand value")?)
                }
                params.push(ASTNode::SymbolLiteral(sym))
            },

            Token::Paren(paren) => {
                if paren == Paren::LeftBrace {
                    // lazy-expression
                    // vec[expression-node]
                    let lazy_expression_node =
                        lazy_expression::resolve(tokens)?;
                    params.push(ASTNode::LazyExpression(
                        lazy_expression_node.into()
                    ));
                } else
                if paren == Paren::LeftBracket {
                    // array literal
                    let array_literal_node =
                        array::literal_resolve(tokens)?;
                    params.push(ASTNode::ArrayLiteral(
                        array_literal_node.into()
                    ));
                } else
                if paren == Paren::LeftParen {
                    // nested expression
                    let current_node =
                        resolve(tokens)?.into();
                    params.push(ASTNode::Expression(current_node));
                } else
                // breaks when encount `)`
                if paren == Paren::RightParen { break }
            },
            Token::Identi(name) => {
                // variable || function invocation || array element reading
                // as compose
                let compose_node =
                    compose::resolve(ASTNode::Variable(
                        VariableNode { name }.into()
                    ), tokens)?;
                params.push(compose_node);
            },

            Token::Keywords(Keyword::Function) => {
                // function definition
                let function_definition =
                    function_definition::resolve(tokens)?;
                params.push(ASTNode::FunctionDefinition(
                    function_definition.into()
                ));
            },
            Token::Keywords(Keyword::Class) => {
                // class definition
                let class_definition =
                    class_definition::resolve(tokens)?;
                params.push(ASTNode::ClassDefinition(
                    class_definition.into()
                ));
            },
            Token::Keywords(Keyword::New) => {
                // class instantiation
                let instantiation_node =
                    instantiation::resolve(tokens)?;
                params.push(ASTNode::Instantiation(
                    instantiation_node.into()
                ));
            },

            _ => {
                let msg = format!("unexpected token {}", current);
                return Err(syntax_error(&msg)?)
            }
        }
    }

    let mut symbol_stack = ASTVec::new();
    let mut result_stack = ASTVec::new();

    for node in params {
        match node {
            // regard the following ASTNode as number
            ASTNode::Variable(_)        |
            ASTNode::Assignment(_)      |
            ASTNode::NumberLiteral(_)   |
            ASTNode::StringLiteral(_)   |
            ASTNode::ArrayLiteral(_)    |
            ASTNode::Expression(_)      |
            ASTNode::Invocation(_)      |
            ASTNode::LazyExpression(_)  |
            ASTNode::ClassDefinition(_) |
            ASTNode::Instantiation(_)   |
            ASTNode::ObjectReading(_)   |
            ASTNode::FunctionDefinition(_) |
            ASTNode::ArrayElementReading(_) =>
                result_stack.push(node),

            ASTNode::SymbolLiteral(_) => {
                if symbol_stack.len() == 0 {
                    symbol_stack.push(node);
                    continue;
                }
                let current_node = &node;
                let mut last_node =
                    symbol_stack.last().unwrap();
                let mut priority =
                    compare(current_node, last_node)?;

                if priority > 1 {
                    // current priority > last priority
                    symbol_stack.push(node);
                } else {
                    while priority <= 0 {
                        let poped_node =
                            symbol_stack.pop().unwrap();
                        result_stack.push(poped_node);

                        let optional_last =
                            symbol_stack.last();
                        if optional_last.is_none() { break }

                        last_node = optional_last.unwrap();
                        priority  =
                            compare(current_node, last_node)?;
                    }
                    symbol_stack.push(current_node.to_owned());
                }
            },
            _ => {
                println!("Invalid expression: unexpected ASTNodeType: {}.", node);
                return Err(())
            }
        }
    }
    // pop the remain elements in the symbol_stack
    // and push them into the result_stack
    while symbol_stack.len() > 0 {
        let last_symbol_node =
            symbol_stack.pop().unwrap();
        result_stack.push(last_symbol_node);
    }
    Ok(ExpressionNode {
        elements: result_stack
    })
}
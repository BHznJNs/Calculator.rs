use crate::public::compile_time::keywords::Keywords;
use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::symbol_priority::compare;
use super::{function_definition, class_definition, instantiation, lazy_expression, array, compose};

pub fn resolve(
    tokens: &mut TokenVec,
    within_paren: bool, // input default with paired-paren
) -> Result<ASTNodeVec, ()> {
    let mut params = ASTNodeVec::new();
    let first_index = 0;

    if within_paren {
        // if `within_paren`, first token should be Paren::LeftParen
        if tokens[first_index] != Token::Paren(Parens::LeftParen) {
            // error msg for debug
            println!("Analyzer error from 'expression_resolve'.");
            return Err(())
        }
        // remove LeftParen
        tokens.pop_front();
    }

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();

        match current {
            Token::Number(number) => {
                params.push(ASTNode {
                    type__: ASTNodeTypes::NumberLiteral(number),
                    params: None,
                });
            },
            Token::String(str) => {
                params.push(ASTNode {
                    type__: ASTNodeTypes::StringLiteral(str),
                    params: None,
                });
            },
            Token::Symbol(symbol) => {
                if symbol == Symbols::Equal {
                    println!("Invalid variable / lazy-expression assignment.");
                    return Err(())
                }
                params.push(ASTNode {
                    type__: ASTNodeTypes::SymbolLiteral(symbol),
                    params: None,
                });
            },
            Token::Identi(name) => {
                // variable || function invocation || array element reading
                // as compose
                let current_node =
                compose::resolve(ASTNode {
                    type__: ASTNodeTypes::Variable(name),
                    params: None,
                }, tokens)?;

                params.push(current_node);
            },
            Token::Paren(paren) => {
                if paren == Parens::LeftBrace {
                    // lazy-expression
                    // vec[expression-node]
                    let current_node =
                        lazy_expression::resolve(tokens)?;
                    params.push(current_node);
                } else
                if paren == Parens::LeftBracket {
                    // array literal
                    let current_node =
                        array::literal_resolve(tokens)?;
                    params.push(current_node);
                } else
                if paren == Parens::LeftParen {
                    // nested expression
                    tokens.insert(first_index, current);
                    let nested_expression =
                        resolve(tokens, true)?;
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(nested_expression),
                    };
                    params.push(current_node);
                } else
                if paren == Parens::RightParen { break }
            },
            Token::Keywords(Keywords::Function) => {
                // function definition
                let function_definition =
                    function_definition::resolve(tokens)?;
                params.push(function_definition);
            },
            Token::Keywords(Keywords::Class) => {
                // class definition
                let class_definition =
                    class_definition::resolve(tokens)?;
                params.push(class_definition);
            },
            Token::Keywords(Keywords::New) => {
                // class instantiation
                let instantiation_node =
                    instantiation::resolve(tokens)?;
                params.push(instantiation_node);
            },
            _ => {
                println!("Unexpected token: '{}'.", current);
                return Err(())
            }
        }
    }

    // postfix expression algorithm
    let mut symbol_stack = ASTNodeVec::new();
    let mut result_stack = ASTNodeVec::new();

    for node in params {
        match node.type__ {
            // regard the following ASTNode as number
            ASTNodeTypes::Variable(_)      |
            ASTNodeTypes::Assignment(_)    |
            ASTNodeTypes::NumberLiteral(_) |
            ASTNodeTypes::StringLiteral(_) |
            ASTNodeTypes::ArrayLiteral     |
            ASTNodeTypes::Expression       |
            ASTNodeTypes::Invocation(_)    |
            ASTNodeTypes::LazyExpression   |
            ASTNodeTypes::ClassDefinition  |
            ASTNodeTypes::Instantiation(_) |
            ASTNodeTypes::ObjectReading(_) |
            ASTNodeTypes::FunctionDefinition(_) |
            ASTNodeTypes::ArrayElementReading(_) =>
                result_stack.push(node),

            ASTNodeTypes::SymbolLiteral(_) => {
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
                println!("Invalid expression: unexpected ASTNodeType: {}.", node.type__);
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

    Ok(result_stack)
}
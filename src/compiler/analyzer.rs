use crate::public::token::{Token, TokenVec};
use crate::public::ast::{ASTNode, ASTNodeVec, ASTNodeTypes};
use crate::public::symbols::Symbols;
use super::symbol_priority::compare;

// input default with paired-paren
fn expression_resolve(
    tokens: &mut TokenVec,
    within_paren: bool,
) -> Result<ASTNodeVec, ()> {
    if tokens.len() == 0 {
        println!("Empty expression.");
        return Err(())
    }

    let mut params = ASTNodeVec::new();
    let first_index = 0;

    if within_paren {
        // if `within_paren`, first token should be Symbols::LeftParen
        if let Token::Symbol(Symbols::LeftParen) = tokens[first_index] {
            println!("Analyzer error.");
            return Err(())
        }
        tokens.remove(first_index);
    }

    while first_index < tokens.len() {
        let current = tokens.remove(first_index);

        match current {
            Token::Unknown => {
                println!("Analyzer error.");
                return Err(())
            },
            Token::Number(number) => {
                params.push(ASTNode {
                    type__: ASTNodeTypes::NumberLiteral(number),
                    params: None,
                });
            },
            Token::Symbol(symbol) => {
                if symbol == Symbols::Equal {
                    println!("Invalid variable / LazyExpression assignment.");
                    return Err(())
                }
                params.push(ASTNode {
                    type__: ASTNodeTypes::SymbolLiteral(symbol),
                    params: None,
                });
            },
            Token::Identi(name) => {
                // should only be variable and function invocation

                // if there is the next token
                // and the next token is LeftParen.
                let is_more_token = tokens.len() > 0;
                if is_more_token {
                    let next_token = tokens.remove(first_index);

                    if next_token == Token::Paren(Symbols::LeftParen) {
                        // function invocation
                        let mut current_node = ASTNode {
                            type__: ASTNodeTypes::Invocation(name.clone()),
                            params: None,
                        };

                        // function params
                        tokens.insert(first_index, next_token);
                        let expression_ast = expression_resolve(tokens, true)?;
                        current_node.params = Some(expression_ast);
                        params.push(current_node);
                        continue;
                    } else
                    if let Token::Symbol(symbol) = next_token {
                        if Symbols::is_equal_symbol(symbol) {
                            let equal_symbol = symbol;
                            let right_hand_nodes = expression_resolve(tokens, false)?;

                            let right_hand_expression =
                            if right_hand_nodes[0].type__ == ASTNodeTypes::LazyExpression {
                                // goto statement assignment
                                right_hand_nodes[0].to_owned()
                            } else {
                                let original = ASTNode {
                                    type__: ASTNodeTypes::Expression,
                                    params: Some(right_hand_nodes)
                                };
                                // variable assignment
                                if equal_symbol == Symbols::Equal {
                                    original
                                } else {
                                    // resolve: += | -= | *= | /=
                                    // separated: + | - | * | /
                                    let separated = equal_symbol.separate();
                                    let variable_node = ASTNode {
                                        type__: ASTNodeTypes::Variable(name.clone()),
                                        params: None,
                                    };
                                    let symbol_node = ASTNode {
                                        type__: ASTNodeTypes::SymbolLiteral(separated),
                                        params: None,
                                    };
                                    ASTNode {
                                        type__: ASTNodeTypes::Expression,
                                        params: Some(vec![variable_node, original, symbol_node])
                                    }
                                }
                            };
                            let current_node = ASTNode {
                                type__: ASTNodeTypes::Assignment(name.clone()),
                                params: Some(vec![right_hand_expression]),
                            };

                            params.push(current_node);
                            continue;
                        } else {
                            // next_token is symbol: + - * /
                            tokens.insert(first_index, next_token);
                        }
                    }
                }

                // variable reading
                params.push(ASTNode {
                    type__: ASTNodeTypes::Variable(name),
                    params: None,
                });
            },
            Token::Paren(paren) => {
                if paren == Symbols::LeftBrace {
                    // Goto-statement
                    // vec[expression-node]

                    let mut sub_tokens = TokenVec::new();
                    while first_index < tokens.len() {
                        if first_index == tokens.len() {
                            println!("Unmatched brace");
                            return Err(())
                        }

                        let current = &tokens[first_index];
                        if let Token::Symbol(Symbols::RightBrace) = current {
                            tokens.remove(first_index);
                            break;
                        }
                        sub_tokens.push(tokens.remove(first_index));
                    }
                    let expression_params =
                        expression_resolve(&mut sub_tokens, false)?;
                    let expression_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(expression_params),
                    };
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::LazyExpression,
                        params: Some(vec![expression_node]),
                    };

                    params.push(current_node);
                } else
                if paren == Symbols::LeftParen {
                    // nested expression
                    tokens.insert(first_index, current);
                    let nested_expression =
                        expression_resolve(tokens, true)?;
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(nested_expression),
                    };
                    params.push(current_node);
                } else
                if paren == Symbols::RightParen { break }
            },
            Token::Keyword(_) => todo!(),
        }
    }

    // postfix expression algorithm
    let mut symbol_stack = ASTNodeVec::new();
    let mut result_stack = ASTNodeVec::new();

    for node in params {
        match node.type__ {
            // regard the following ASTNode as number
            ASTNodeTypes::Variable(_) => result_stack.push(node),
            ASTNodeTypes::Assignment(_) => result_stack.push(node),
            ASTNodeTypes::NumberLiteral(_) => result_stack.push(node),
            ASTNodeTypes::Expression => result_stack.push(node),
            ASTNodeTypes::Invocation(_) => result_stack.push(node),
            ASTNodeTypes::LazyExpression => result_stack.push(node),

            ASTNodeTypes::SymbolLiteral(_) => {
                if symbol_stack.len() == 0 {
                    symbol_stack.push(node);
                    continue;
                }
                let current_node = &node;
                let mut last_node = symbol_stack.last().unwrap();
                let mut priority = compare(current_node, last_node)?;

                if priority > 1 {
                    // current priority > last priority
                    symbol_stack.push(node);
                } else {
                    while priority <= 0 {
                        let poped_node = symbol_stack.pop().unwrap();
                        result_stack.push(poped_node);

                        let optional_last = symbol_stack.last();
                        if optional_last.is_none() {
                            break;
                        }

                        last_node = optional_last.unwrap();
                        priority = compare(current_node, last_node)?;
                    }
                    symbol_stack.push(current_node.to_owned());
                }
            },
            _ => {
                println!("Invalid expression.");
                return Err(())
            }
        }
    }
    // pop the remain elements in the symbol_stack
    // and push them into the result_stack
    while symbol_stack.len() > 0 {
        let last_symbol_node = symbol_stack.pop().unwrap();
        result_stack.push(last_symbol_node);
    }

    Ok(result_stack)
}

pub fn analyzer(mut tokens: TokenVec) -> Result<ASTNode, ()> {
    let mut root = ASTNode {
        type__: ASTNodeTypes::Root,
        params: None,
    };
    let mut params = ASTNodeVec::new();

    if tokens.len() == 0 {
        // blank line || line comment
        params.push(ASTNode {
            type__: ASTNodeTypes::Comment,
            params: None,
        });
        
    } else
    if let Token::Keyword(_) = tokens[0] {
        // regard the whole line as a statement
        let current_node = ASTNode {
            type__: ASTNodeTypes::Statement,
            params: None,
        };
        params.push(current_node);
    } else {
        // regard the whole line as a expression
        let expression_nodes = expression_resolve(
            &mut tokens, false
        )?;

        let current_node = ASTNode {
            type__: ASTNodeTypes::Expression,
            params: Some(expression_nodes),
        };

        params.push(current_node);
    }
    root.params = Some(params);
    Ok(root)
}
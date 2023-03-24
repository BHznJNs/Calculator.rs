use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::symbol_priority::compare;
use super::sequence_resolve::sequence_resolve;

fn lazy_expression_resolve(
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    let first_index = 0;
    let mut sub_tokens = TokenVec::new();
    let mut brace_count = 1;

    while first_index < tokens.len() {
        if first_index == tokens.len() {
            println!("Unmatched brace.");
            return Err(())
        }

        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Parens::LeftBrace) {
            brace_count += 1;
        }
        if current == Token::Paren(Parens::RightBrace) {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }

    let sub_sequence = sequence_resolve(&mut sub_tokens)?;
    let current_node = ASTNode {
        type__: ASTNodeTypes::LazyExpression,
        params: Some(vec![sub_sequence]),
    };

    Ok(current_node)
}

fn array_literal_resolve(
    tokens: &mut TokenVec
) -> Result<ASTNode, ()> {
    let first_index = 0;
    let mut elements = ASTNodeVec::new();
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();

        let is_divider =
            current == Token::Divider;
        let is_right_bracket =
            current == Token::Paren(Parens::RightBracket);
        
        if current == Token::Paren(Parens::LeftBracket) {
            // sub array element
            let sub_array_node =
                array_literal_resolve(tokens)?;
            elements.push(sub_array_node);
        } else if is_divider || is_right_bracket {
            // number element
            if sub_tokens.len() > 0 {
                let sub_expression =
                    expression_resolve(&mut sub_tokens, false)?;
                sub_tokens.clear();

                let sub_expression_node = ASTNode {
                    type__: ASTNodeTypes::Expression,
                    params: Some(sub_expression),
                };
                elements.push(sub_expression_node);
            }
            if is_right_bracket { break }
        } else {
            sub_tokens.push_back(current);
        }
    }

    let array_node = ASTNode {
        type__: ASTNodeTypes::ArrayLiteral,
        params: Some(elements),
    };

    Ok(array_node)
}

// input default with paired-paren
pub fn expression_resolve(
    tokens: &mut TokenVec,
    within_paren: bool,
) -> Result<ASTNodeVec, ()> {
    let mut params = ASTNodeVec::new();
    let first_index = 0;

    if within_paren {
        // if `within_paren`, first token should be Paren::LeftParen
        if tokens[first_index] != Token::Paren(Parens::LeftParen) {
            println!("Analyzer error from 'expression_resolve'.");
            return Err(())
        }
        tokens.remove(first_index);
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
                // should only be variable and function invocation

                // if there is the next token
                // and the next token is LeftParen.
                let is_more_token = tokens.len() > 0;
                if is_more_token {
                    let next_token = tokens.pop_front().unwrap();

                    if next_token == Token::Paren(Parens::LeftParen) {
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
                    if next_token == Token::Paren(Parens::LeftBracket) {
                        // array element reading
                        let mut sub_tokens = TokenVec::new();
                        while first_index < tokens.len() {
                            let current = tokens.pop_front().unwrap();
                            if current == Token::Paren(Parens::RightBracket) {
                                break;
                            }
                            sub_tokens.push_back(current);
                        }
                        let reading_index_expression =
                            expression_resolve(&mut sub_tokens, false)?;
                        params.push(ASTNode {
                            type__: ASTNodeTypes::ArrayElementReading(name),
                            params: Some(reading_index_expression),
                        });
                        continue;
                    } else
                    if let Token::Symbol(symbol) = next_token {
                        if Symbols::is_equal_symbol(symbol) {
                            // assignment
                            // symbols: += | -= | *= | /= | ^=
                            let equal_symbol = symbol;
                            let right_hand_nodes =
                                expression_resolve(tokens, false)?;

                            if right_hand_nodes.len() == 0 {
                                println!("Invalid assignment.");
                                return Err(())
                            }

                            let right_hand_expression =
                            if right_hand_nodes[0].type__ == ASTNodeTypes::LazyExpression ||
                               right_hand_nodes[0].type__ == ASTNodeTypes::ArrayLiteral {
                                // lazy-expression || array assignment
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
                                    // resolve:   += | -= | *= | /= | ^=
                                    // separated: +  | -  | *  | /  | ^
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
                                type__: ASTNodeTypes::Assignment(name),
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
                if paren == Parens::LeftBrace {
                    // lazy-expression
                    // vec[expression-node]
                    let current_node =
                        lazy_expression_resolve(tokens)?;
                    params.push(current_node);
                } else
                if paren == Parens::LeftBracket {
                    // array literal
                    let current_node =
                        array_literal_resolve(tokens)?;
                    params.push(current_node);
                } else
                if paren == Parens::LeftParen {
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
                if paren == Parens::RightParen { break }
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
            ASTNodeTypes::ArrayLiteral     |
            ASTNodeTypes::Expression       |
            ASTNodeTypes::Invocation(_)    |
            ASTNodeTypes::LazyExpression   |
            ASTNodeTypes::ArrayElementReading(_) => result_stack.push(node),

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
use crate::public::compile_time::keywords::Keywords;
use crate::public::value::parens::Parens;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::compiler::tokenizer::token::{Token, TokenVec};

use super::symbol_priority::compare;
use super::{sequence_resolve, function_definition};

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

    let sub_sequence =
        sequence_resolve::resolve(&mut sub_tokens)?;
    let current_node = ASTNode {
        type__: ASTNodeTypes::LazyExpression,
        params: Some(vec![sub_sequence]),
    };

    Ok(current_node)
}

fn func_params_resolve(
    tokens: &mut TokenVec
) -> Result<ASTNodeVec, ()> {
    // examples:
    // 1, 2)
    // a, 1)

    fn param_expr_resolve(
        sub_tokens: &mut TokenVec,
        params: &mut ASTNodeVec,
    ) -> Result<(), ()> {
        if sub_tokens.len() > 0 {
            let sub_expression =
                resolve(sub_tokens, false)?;
            let sub_expression_node = ASTNode {
                type__: ASTNodeTypes::Expression,
                params: Some(sub_expression),
            };
            params.push(sub_expression_node);
            sub_tokens.clear();
        }
        Ok(())
    }

    let first_index = 0;
    let mut paren_count = 1;
    let mut sub_tokens = TokenVec::new();
    let mut params = ASTNodeVec::new();

    while first_index < tokens.len() {
        if first_index == tokens.len() {
            println!("Unmatched brace.");
            return Err(())
        }

        let current = tokens.pop_front().unwrap();

        let is_divider =
            current == Token::Divider;
        let is_left_paren =
            current == Token::Paren(Parens::LeftParen);
        let is_right_paren =
            current == Token::Paren(Parens::RightParen);

        if is_left_paren {
            paren_count += 1;
        }
        if is_divider {
            param_expr_resolve(&mut sub_tokens, &mut params)?;
        }
        if is_right_paren {
            paren_count -= 1;
            if paren_count == 0 {
                param_expr_resolve(&mut sub_tokens, &mut params)?;
                break;
            }
        }

        if !is_divider {
            sub_tokens.push_back(current);
        }
    }

    Ok(params)
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
                    resolve(&mut sub_tokens, false)?;
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

fn array_reading_resolve(
    arr_name: String,
    tokens: &mut TokenVec,
) -> Result<ASTNode, ()> {
    // example:
    // 1] | from `arr[1]`
    // 1][2] | from `arr[1][2]`
    let first_index = 0;
    let mut bracket_count = 1;
    let mut params = ASTNodeVec::new();
    let mut sub_tokens = TokenVec::new();

    while first_index < tokens.len() {
        let current = tokens.pop_front().unwrap();
        if current == Token::Paren(Parens::LeftBracket) {
            bracket_count += 1;
        }
        if current == Token::Paren(Parens::RightBracket) {
            bracket_count -= 1;
            if bracket_count == 0 {
                break;
            }
        }
        sub_tokens.push_back(current);
    }
    let reading_index_expression_nodes =
        resolve(&mut sub_tokens, false)?;
    params.push(ASTNode {
        type__: ASTNodeTypes::Expression,
        params: Some(reading_index_expression_nodes),
    });

    // sub array element reading
    let next_token = tokens.pop_front();
    if let Some(Token::Paren(Parens::LeftBracket)) = next_token {
        let sub_element_reading =
            array_reading_resolve(arr_name.clone(), tokens)?;
        params.push(sub_element_reading);
    } else if next_token == None {
        // empty
    } else {
        // exist next_token and next_token
        // is not equal Token::Paren(Parens::LeftBracket)
        tokens.push_front(next_token.unwrap())
    }

    let current_node = ASTNode {
        type__: ASTNodeTypes::ArrayElementReading(arr_name),
        params: Some(params)
    };
    Ok(current_node)
}

// --- --- --- --- --- ---

pub fn resolve(
    tokens: &mut TokenVec,
    within_paren: bool, // input default with paired-paren
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

                // if there is the next token
                // and the next token is LeftParen.
                let is_more_token = tokens.len() > 0;
                if is_more_token {
                    let next_token = tokens.pop_front().unwrap();

                    if next_token == Token::Paren(Parens::LeftParen) {
                        // function invocation
                        let invoke_params =
                            func_params_resolve(tokens)?;
                        let current_node = ASTNode {
                            type__: ASTNodeTypes::Invocation(name.clone()),
                            params: Some(invoke_params),
                        };
                        params.push(current_node);
                        continue;
                    } else
                    if next_token == Token::Paren(Parens::LeftBracket) {
                        // array element reading
                        let array_reading_node =
                            array_reading_resolve(name, tokens)?;
                        params.push(array_reading_node);
                        continue;
                    } else
                    if let Token::Symbol(symbol) = next_token {
                        if Symbols::is_equal_symbol(symbol) {
                            // assignment
                            // symbols: += | -= | *= | /= | ^=
                            let equal_symbol = symbol;
                            let mut right_hand_nodes =
                                resolve(tokens, false)?;

                            if right_hand_nodes.len() == 0 {
                                println!("Invalid assignment.");
                                return Err(())
                            }

                            let right_hand_expression =
                            if let ASTNodeTypes::LazyExpression |
                                   ASTNodeTypes::ArrayLiteral   |
                                   ASTNodeTypes::FunctionDefinition(_) =
                                   right_hand_nodes[0].type__ {
                                // lazy-expression || array assignment || function
                                right_hand_nodes.remove(0)
                                // right_hand_nodes[0].to_owned()
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
                        resolve(tokens, true)?;
                    let current_node = ASTNode {
                        type__: ASTNodeTypes::Expression,
                        params: Some(nested_expression),
                    };
                    params.push(current_node);
                } else
                if paren == Parens::RightParen { break }
            },
            Token::Keywords(Keywords::Fn) => {
                // function assignment
                let function_definition =
                    function_definition::resolve(tokens)?;
                params.push(function_definition);
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
            ASTNodeTypes::FunctionDefinition(_) | // test
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
                println!("Invalid expression: unexpected ASTNodeType: {}.", node.type__);
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
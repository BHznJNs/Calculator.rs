use std::collections::HashMap;

use crate::public::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::number::Number;
use crate::public::symbols::Symbols;

fn operate(num1: Number, num2: Number, operator: Symbols) -> Result<Number, ()> {
    match operator {
        Symbols::Plus     => {Ok(num1 + num2)},
        Symbols::Minus    => {Ok(num1 - num2)},
        Symbols::Multiply => {Ok(num1 * num2)},
        Symbols::Divide   => {Ok(num1 / num2)},
        Symbols::Power    => {Ok(num1.pow(num2))},
        _                 => {
            println!("Unexpected symbol: '{}' at function `operate`.", operator);
            return Err(())
        },
    }
}

fn expression_compute(
    expression_node: &ASTNode,
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>
) -> Result<Number, ()> {
    let params = expression_node
        .params
        .as_ref()
        .unwrap();
    let mut number_stack = Vec::<Number>::new();
    let mut symbol_stack = Vec::<Symbols>::new();
    let mut index = 0;

    while index < params.len() {
        let node = &params[index];

        match &node.type__ {
            ASTNodeTypes::Variable(name) => {
                let optional_var = variables.get(name.as_str());
                if optional_var.is_none() {
                    println!("Undefined variable: '{}'.", name);
                    return Err(())
                }
                number_stack.push(*optional_var.unwrap());
            },
            ASTNodeTypes::NumberLiteral(number) => number_stack.push(*number),
            ASTNodeTypes::SymbolLiteral(symbol) => {
                let symbol_self = *symbol;
                let last_symbol = if let Some(symbol) = symbol_stack.last() {
                    *symbol
                } else {
                    Symbols::NotASymbol
                };

                if (symbol_self as i8) <= (last_symbol as i8) {
                    // e.g.
                    // last   : +
                    // current: -
                    // --- --- --- ---
                    // last   : ^
                    // current: *
                    
                    symbol_stack.push(*symbol);
                } else {
                    // e.g.
                    // last   : +
                    // current: *

                    // put the higher_priority symbol to the last
                    let higher_priority = symbol_stack.pop().unwrap();
                    symbol_stack.push(symbol_self);
                    symbol_stack.push(higher_priority);

                    // --- --- --- --- --- ---

                    index += 1;
                    if index == params.len() {
                        println!("Invalid expression.");
                        return Err(())
                    }
                    // next node should be a Number
                    let next_node_type = &params[index].type__;
                    let next_number = if
                        let ASTNodeTypes::NumberLiteral(num) = next_node_type
                    { *num } else {
                        println!("Expected NumberLiteral, found {}.", next_node_type);
                        return Err(())
                    };

                    // pop the last two elements
                    // [..., num1, num2]
                    // [...] num1 num2
                    // push the next number and the poped elements
                    // [..., num1, next_number, num2]
                    let num2 = number_stack.pop().unwrap();
                    let num1 = number_stack.pop().unwrap();
                    number_stack.push(num2);
                    number_stack.push(next_number);
                    number_stack.push(num1);
                }
            },
            ASTNodeTypes::Expression => {
                let express_res = expression_compute(
                    node,
                    build_in_funcs,
                    variables,
                    goto_statements,
                )?;
                number_stack.push(express_res);
            },
            ASTNodeTypes::GotoStatement => {
                let expression_node = &node
                    .params
                    .as_ref()
                    .unwrap()[0];

                let expression_value = expression_compute(
                    expression_node,
                    build_in_funcs,
                    variables,
                    goto_statements
                )?;
                number_stack.push(expression_value);
            },
            ASTNodeTypes::InvokeExpression(name) => {
                let func_res: Number;

                // prioritize using build-in function
                match build_in_funcs.get(name.as_str()) {
                    Some(func) => {
                        // build-in function
                        if let Some(params) = &node.params {
                            if params.len() == 0 {
                                println!("Parameter for function '{}' is missing.", name);
                                return Err(())
                            }

                            let expression_node = ASTNode {
                                type__: ASTNodeTypes::Expression,
                                params: Some(params.to_vec()),
                            };

                            let expression_value = expression_compute(
                                &expression_node,
                                build_in_funcs,
                                variables,
                                goto_statements,
                            )?;
                            let func_result_f = match expression_value {
                                Number::NotANumber => {
                                    println!("Not A Number Error.");
                                    return Err(())
                                },
                                Number::Int(i) => func(i as f64),
                                Number::Float(f) => func(f),
                            };
                            func_res = Number::Float(func_result_f);
                        } else {
                            println!("Analyzer error.");
                            return Err(())
                        }
                    },
                    None => match goto_statements.get(name) {
                        // user defined goto-statement
                        Some(func) => {
                            let mut ast_vec = ASTNodeVec::new();
                            ast_vec.push(func.clone());
                            let root_node = ASTNode {
                                type__: ASTNodeTypes::Root,
                                params: Some(ast_vec),
                            };
                            func_res = compute(
                                root_node,
                                build_in_funcs,
                                variables,
                                goto_statements
                            )?;
                        },
                        None => {
                            println!("Undefined function: '{}'.", name);
                            return Err(())
                        },
                    },
                }
                number_stack.push(func_res);
            },
            _ => {
                println!("Unexpected node type: '{}'.", node.type__);
                return Err(())
            }
        }
        index += 1;
    }

    // LOG
    // for n in &number_stack {
    //     println!("num: {}", n);
    // }
    // for s in &symbol_stack {
    //     println!("sym: {}", s);
    // }

    let first_index = 0;
    while first_index < symbol_stack.len() {
        if number_stack.len() < 2 {
            println!("Invalid expression.");
            return Err(())
        }

        let symbol = symbol_stack.remove(first_index);
        let num1 = number_stack.remove(first_index);
        let num2 = number_stack.remove(first_index);
        let value = operate(num1, num2, symbol)?;
        number_stack.push(value);
    }

    Ok(number_stack[0])
}

pub fn compute(
    root_node: ASTNode,
    build_in_funcs:  &HashMap<&str, fn(f64) -> f64>,
    variables:       &mut HashMap<String, Number>,
    goto_statements: &mut HashMap<String, ASTNode>
) -> Result<Number, ()> {
    /*
        Root {
          Expression {
             Number,
             Symbol,
             Expression,
             ...
          }
        }
     */
    /*
        Root {
          Expression {
             Number,
             Symbol,
             Expression,
             ...
          }
        }
     */


    let params = root_node
        .params
        .as_ref()
        .unwrap();

    let param_element = &params[0];

    if param_element.type__ == ASTNodeTypes::Expression {
        let expression_res = expression_compute(
            &param_element,
            build_in_funcs,
            variables,
            goto_statements,
        )?;

        return Ok(expression_res);
    } else if let ASTNodeTypes::Assignment(name) = &param_element.type__ {
        let right_hand = &param_element
            .params
            .as_ref()
            .unwrap()[0];

        if right_hand.type__ == ASTNodeTypes::Expression {
            let expression_value = expression_compute(
                right_hand,
                build_in_funcs,
                variables,
                goto_statements
            )?;
            variables.insert(name.clone(), expression_value);
            Ok(expression_value)
        } else if right_hand.type__ == ASTNodeTypes::GotoStatement {
            let sub_expression = &right_hand
                .params
                .as_ref()
                .unwrap()[0];

            goto_statements.insert(
                name.clone(),
                sub_expression.clone()
            );
            let expression_value = expression_compute(
                sub_expression,
                build_in_funcs,
                variables,
                goto_statements,
            )?;

            Ok(expression_value)
        } else {
            println!("Analyzer error.");
            return Err(())
        }
    } else {
        println!("Analyzer error.");
        return Err(())
    }
}
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::resolvers::array_reading_resolve::array_reading_resolve;
use super::resolvers::sequence_resolve::sequence_resolve;
use super::resolvers::assignment_resolve::assignment_resolve;
use super::operate::operate;

pub fn expression_compute(
    expression_node: &ASTNode,
    global: &mut Global,
) -> Result<Box<Value>, ()> {
    let params = expression_node
        .params
        .as_ref()
        .unwrap();

    let mut value_stack = Vec::<Box::<Value>>::new();
    let mut index = 0;

    while index < params.len() {
        let node = &params[index];

        match &node.type__ {
            ASTNodeTypes::Expression => {
                let express_res = expression_compute(
                    node, global
                )?;
                value_stack.push(express_res);
            },

            ASTNodeTypes::Variable(name) => {
                let optional_var =
                    global.variables.get(name.as_str());
                let variable_value = match optional_var {
                    Some(val) => val,
                    None => {
                        println!("Undefined variable: '{}'.", name);
                        return Err(())
                    },
                };
                value_stack.push(variable_value.to_owned());
            },
            ASTNodeTypes::NumberLiteral(number) =>
                value_stack.push(Box::new(Value::Number(*number))),
            ASTNodeTypes::SymbolLiteral(symbol) => {
                if value_stack.len() < 2 {
                    println!("Invalid expression: operating number is missing.");
                    return Err(())
                }
                let num2 = value_stack.pop().unwrap();
                let num1 = value_stack.pop().unwrap();
                let current_symbol = *symbol;
                let value = operate(num1, num2, current_symbol)?;

                value_stack.push(Box::new(value));
            },
            ASTNodeTypes::ArrayElementReading(_) => {
                let target_element = array_reading_resolve(
                    node, None,
                    true, global
                )?;
                value_stack.push(target_element);
            },
            ASTNodeTypes::Assignment(name) => {
                let right_hand = &node
                    .params
                    .as_ref()
                    .unwrap()[0];

                let assignment_res = 
                    assignment_resolve(name, right_hand, global)?;
                value_stack.push(assignment_res);
            },
            ASTNodeTypes::Invocation(name) => {
                let func_result: Box<Value>;

                let optional_func =
                    global.build_in_funcs.get(name.as_str());
                // prioritize using build-in function
                match optional_func {
                    Some(f) => {
                        let func = f.clone();
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

                            let expression_value =
                                expression_compute(&expression_node, global)?;
                            if let Value::Number(num) = expression_value.as_ref() {
                                let func_result_f = match num {
                                    Number::Int(i) => func(*i as f64),
                                    Number::Float(f) => func(*f),
                                    _ => {
                                        println!("Not A Number Error.");
                                        return Err(())
                                    },
                                };
                                func_result = Box::new(Value::Number(Number::Float(func_result_f)));
                            } else {
                                println!("Invalid params for function '{}'.", name);
                                return Err(())
                            }
                        } else {
                            println!("Analyzer error from 'expression_compute'.");
                            return Err(())
                        }
                    },
                    None => match global.variables.get(name) {
                        // user defined LazyExpression
                        // le => LazyExpression
                        Some(value_box) => {
                            let le_value = value_box.as_ref();
                            if let Value::LazyExpression(le) = le_value {
                                func_result =
                                    sequence_resolve(le, global)?;
                            } else {
                                println!("'{}' is not a lazy-expression.", name);
                                return Err(())
                            }
                        },
                        _ => {
                            println!("Undefined function OR lazy-expression: '{}'.", name);
                            return Err(())
                        },
                    },
                }
                value_stack.push(func_result);
            },
            _ => {
                println!("Unexpected node type: '{}'.", node.type__);
                return Err(())
            }
        }
        index += 1;
    }

    Ok(value_stack.remove(0))
}
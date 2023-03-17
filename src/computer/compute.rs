use crate::public::ast::{ASTNode, ASTNodeTypes, ASTNodeVec};
use crate::public::global::Global;
use crate::public::number::Number;
use super::operate::operate;

fn assignment_resolve(
    name: &String,
    right_hand: &ASTNode,
    global: &mut Global
) -> Result<Number, ()> {
    if let ASTNodeTypes::Expression = right_hand.type__ {
        // variable assignment
        let expression_value = expression_compute(
            right_hand, global
        )?;
        global.variables.insert(name.clone(), expression_value);
        return Ok(expression_value)
    } else if let ASTNodeTypes::LazyExpression = right_hand.type__ {
        // LazyExpression assignment
        let sub_expression = &right_hand
            .params
            .as_ref()
            .unwrap()[0];

        global.lazy_expressions.insert(
            name.clone(),
            sub_expression.to_owned()
        );
        return Ok(Number::Empty)
    } else {
        println!("Analyzer error.");
        return Err(())
    }
}

fn expression_compute(
    expression_node: &ASTNode,
    global: &mut Global,
) -> Result<Number, ()> {
    let params = expression_node
        .params
        .as_ref()
        .unwrap();
    let mut number_stack = Vec::<Number>::new();
    let mut index = 0;

    while index < params.len() {
        let node = &params[index];

        match &node.type__ {
            ASTNodeTypes::Variable(name) => {
                let optional_var =
                    global.variables.get(name.as_str());
                if optional_var.is_none() {
                    println!("Undefined variable: '{}'.", name);
                    return Err(())
                }
                number_stack.push(*optional_var.unwrap());
            },
            ASTNodeTypes::NumberLiteral(number) => number_stack.push(*number),
            ASTNodeTypes::SymbolLiteral(symbol) => {
                if number_stack.len() < 2 {
                    println!("Invalid expression.");
                    return Err(())
                }
                let num2 = number_stack.pop().unwrap();
                let num1 = number_stack.pop().unwrap();
                let current_symbol = *symbol;
                let value = operate(num1, num2, current_symbol)?;

                number_stack.push(value);
            },
            ASTNodeTypes::Expression => {
                let express_res = expression_compute(
                    node, global
                )?;
                number_stack.push(express_res);
            },
            ASTNodeTypes::Assignment(name) => {
                let right_hand = &node
                    .params
                    .as_ref()
                    .unwrap()[0];

                let assignment_res = assignment_resolve(
                    name, right_hand, global
                )?;
                number_stack.push(assignment_res);
            },
            ASTNodeTypes::Invocation(name) => {
                let func_res: Number;

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

                            let expression_value = expression_compute(
                                &expression_node, global
                            )?;
                            let func_result_f = match expression_value {
                                Number::Int(i) => func(i as f64),
                                Number::Float(f) => func(f),
                                _ => {
                                    println!("Not A Number Error.");
                                    return Err(())
                                },
                            };
                            func_res = Number::Float(func_result_f);
                        } else {
                            println!("Analyzer error.");
                            return Err(())
                        }
                    },
                    None => match global.lazy_expressions.get(name) {
                        // user defined LazyExpression
                        Some(func) => {
                            let mut ast_vec = ASTNodeVec::new();
                            ast_vec.push(func.clone());
                            let root_node = ASTNode {
                                type__: ASTNodeTypes::Root,
                                params: Some(ast_vec),
                            };
                            func_res = compute(
                                root_node, global
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

    Ok(number_stack[0])
}

pub fn compute(
    root_node: ASTNode,
    global: &mut Global,
) -> Result<Number, ()> {
    /*
        Root {
          Expression {
            Assignment,
            Symbol,
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

    if let ASTNodeTypes::Expression = param_element.type__ {
        let expression_res = expression_compute(
            &param_element,
            global,
        )?;
        Ok(expression_res)
    } else if let ASTNodeTypes::Comment = param_element.type__ {
        Ok(Number::Empty)
    } else {
        Err(())
    }
}
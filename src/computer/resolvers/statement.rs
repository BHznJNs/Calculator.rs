use std::io::stdout;
use std::rc::Rc;

use crate::computer::resolvers::expression;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::StatementNode;
use crate::public::compile_time::keywords::Keyword;
use crate::public::error::syntax_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, VoidSign};
use crate::utils::output::print_line;

use super::sequence;

pub fn resolve(statement_node: Rc<StatementNode>, scope: &mut Scope) -> Result<Value, ()> {
    let (condition, body) = (
        statement_node.condition.clone().take(),
        &statement_node.body,
    );

    let result = match statement_node.keyword {
        Keyword::Out => {
            let output_value = if let Some(ASTNode::Expression(expression_node)) = body.get(0) {
                expression::resolve(expression_node.clone(), scope)?
            } else {
                Value::Void(VoidSign::Empty)
            };

            print_line(&mut stdout(), output_value);
            // println!("{}", output_value);
            Value::Void(VoidSign::Empty)
        }
        Keyword::For => {
            let loop_count_expression = condition.unwrap();
            let loop_count_value = expression::resolve(loop_count_expression.into(), scope)?;

            let is_inf_loop;
            let loop_count;
            match loop_count_value {
                Value::Number(num) => {
                    is_inf_loop = false;
                    loop_count = num.int_value();
                }
                Value::Void(VoidSign::Empty) => {
                    is_inf_loop = true;
                    loop_count = 0;
                }
                _ => return Err(syntax_error("invalid loop count for 'for' statement")?),
            }

            let mut count = 0;
            'outer: loop {
                // these is used to control loop times
                if !is_inf_loop {
                    if count == loop_count {
                        break;
                    }
                    count += 1;
                }

                // --- --- --- --- --- ---

                'inner: for sequence in body {
                    let sequence_clone = sequence.clone();
                    let sequence_result = sequence::resolve(sequence_clone.into(), scope)?;

                    if let Value::Void(sign) = sequence_result {
                        if let VoidSign::Break(_) = sign {
                            // encount `break` | `brk`
                            break 'outer;
                        }
                        if sign == VoidSign::Continue {
                            // encount `continue` | `ctn`
                            break 'inner;
                        }
                    }
                }
            }

            Value::Void(VoidSign::Empty)
        }
        Keyword::If => {
            let condition_struct = expression::resolve(condition.unwrap().into(), scope)?;
            let condition_value = match condition_struct {
                Value::Boolean(val) => val,
                Value::Number(num) => num.int_value() != 0,
                _ => return Err(syntax_error("invalid condition for 'if' statement")?),
            };

            if condition_value {
                for sequence in body {
                    let sequence_clone = sequence.clone();
                    let sequence_result = sequence::resolve(sequence_clone.into(), scope)?;

                    if let Value::Void(_) = sequence_result {
                        return Ok(sequence_result);
                    }
                }
            }

            Value::Void(VoidSign::Empty)
        }

        Keyword::Continue => Value::Void(VoidSign::Continue),
        Keyword::Break => {
            if let Some(ASTNode::Expression(expression_node)) = body.get(0) {
                let expression_res = expression::resolve(expression_node.clone(), scope)?;
                Value::Void(VoidSign::Break(expression_res.into()))
            } else {
                Value::Void(VoidSign::Empty)
            }
        }
        _ => Value::Void(VoidSign::Empty),
    };
    Ok(result)
}

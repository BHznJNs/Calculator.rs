use std::io::stdout;

use crate::computer::resolvers::{assignment, expression};
use crate::public::compile_time::ast::types::StatementNode;
use crate::public::error::syntax_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, VoidSign};
use crate::utils::output::print_line;

use super::sequence;

pub fn resolve(statement_node: &StatementNode, scope: &mut Scope) -> Result<Value, ()> {
    let result = match statement_node {
        StatementNode::Output(expression_node) => {
            let output_value = expression::resolve(expression_node, scope)?;
            print_line(&mut stdout(), output_value);
            Value::Void(VoidSign::Empty)
        }
        StatementNode::ForLoop(for_statement) => {
            let loop_count_value = expression::resolve(&for_statement.loop_count, scope)?;

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

                'inner: for sequence in &for_statement.body {
                    let sequence_result = sequence::resolve(sequence, scope)?;

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
        StatementNode::Condition(if_statement) => {
            let condition_value = expression::resolve(&if_statement.condition, scope)?;

            if condition_value.get_bool() {
                for sequence in &if_statement.body {
                    let sequence_result = sequence::resolve(sequence, scope)?;

                    if let Value::Void(_) = sequence_result {
                        return Ok(sequence_result);
                    }
                }
            }

            Value::Void(VoidSign::Empty)
        }
        StatementNode::Import(import_node) => {
            // import_node.type__ must be `ModuleType::BuildIn`
            scope.import_std(&import_node.target)?;
            Value::Void(VoidSign::Empty)
        }
        StatementNode::GlobalAssignment(assignment_node) => {
            assignment::resolve(assignment_node, scope, true)?
        }

        StatementNode::Continue => Value::Void(VoidSign::Continue),
        StatementNode::Break(expression_node) => {
            let expression_value = expression::resolve(expression_node, scope)?;
            if expression_value == Value::Void(VoidSign::Empty) {
                Value::Void(VoidSign::Empty)
            } else {
                Value::Void(VoidSign::Break(expression_value.into()))
            }
        }
    };
    return Ok(result);
}

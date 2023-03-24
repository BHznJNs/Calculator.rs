use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::array_resolve::array_resolve;
use super::super::expression_compute::expression_compute;

pub fn assignment_resolve(
    name: &String,
    right_hand: &ASTNode,
    global: &mut Global
) -> Result<Value, ()> {
    let result = match right_hand.type__ {
        ASTNodeTypes::Expression => {
            let expression_value =
                expression_compute(right_hand, global)?;

            global.variables.insert(
                name.clone(),
                expression_value.clone(),
            );
            expression_value
        },
        ASTNodeTypes::LazyExpression => {
            let sub_expression = &right_hand
                .params
                .as_ref()
                .unwrap()[0];

            global.variables.insert(
                name.clone(),
                Value::LazyExpression(sub_expression.to_owned())
            );
            Value::Number(Number::Empty(None))
        },
        ASTNodeTypes::ArrayLiteral => {
            let array =
                array_resolve(right_hand, global)?;

            global.variables.insert(name.clone(), array);
            Value::Number(Number::Empty(None))
        },

        _ => {
            println!("Analyzer error from 'assignment_resolve'.");
            return Err(())
        }
    };
    Ok(result)
}
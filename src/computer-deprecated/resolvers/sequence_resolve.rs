use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::global::Global;
use crate::public::value::number::Number;
use crate::public::value::value::Value;

use super::super::expression_compute::expression_compute;
use super::statement_resolve::statement_resolve;

pub fn sequence_resolve(
    sequence: &ASTNode,
    global: &mut Global,
) -> Result<Box<Value>, ()> {
    let result = match sequence.type__ {
        ASTNodeTypes::Expression => {
            let expression_result =
                expression_compute(sequence, global)?;

            expression_result
        },
        ASTNodeTypes::Statement(keyword) => {
            let optional_keyword =
                statement_resolve(keyword, sequence, global)?;
            Box::new(Value::Number(Number::Empty(optional_keyword)))
        },
        // ASTNodeTypes::Comment => {},
        _ => Box::new(Value::Number(Number::Empty(None)))
    };
    Ok(result)
}
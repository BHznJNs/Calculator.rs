use crate::public::ast::{ASTNode, ASTNodeTypes};
use crate::public::global::Global;
use crate::public::value::number::Number;

use super::expression_compute::expression_compute;
use super::statement_resolve::statement_resolve;

pub fn sequence_resolve(
    sequence: &ASTNode,
    global: &mut Global,
) -> Result<Number, ()> {
    match sequence.type__ {
        ASTNodeTypes::Expression => {
            let expression_result =
                expression_compute(sequence, global)?;

            Ok(expression_result)
        },
        ASTNodeTypes::Statement(keyword) => {
            let optional_keyword =
                statement_resolve(keyword, sequence, global)?;
            Ok(Number::Empty(optional_keyword))
        },
        // ASTNodeTypes::Comment => {},
        _ => Ok(Number::Empty(None))
    }
}
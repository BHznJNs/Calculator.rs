use crate::public::ast::{ASTNode, ASTNodeTypes};
use crate::public::global::Global;
use crate::public::number::Number;

use super::expression_compute::expression_compute;

pub fn assignment_resolve(
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
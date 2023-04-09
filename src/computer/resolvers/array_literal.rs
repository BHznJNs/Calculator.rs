use super::expression;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::ArrayLiteral;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};

pub fn resolve(
    array_node: &ASTNode,
    scope: &mut Scope,
) -> Result<ArrayLiteral, ()> {
    let params =
        array_node.params
        .as_ref()
        .unwrap();
    let mut elements = ArrayLiteral::new();

    for node in params {
        match node.type__ {
            ASTNodeTypes::Expression => {
                let expression_value_rc =
                    expression::resolve(&node, scope)?;

                let expression_value =
                    expression_value_rc.unwrap();
                elements.push_back(expression_value)
            },
            _ => {
                println!("Invalid array element.");
                return Err(())
            }
        }
    }

    Ok(elements)
}
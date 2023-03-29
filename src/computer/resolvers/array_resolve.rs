use super::expression_resolve;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::{Value, ArrayLiteral, Overload};
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
                // The expr result must be Rc<Value::Number>,
                // so clone it costs low performance.
                let expression_value =
                    expression_resolve::resolve(&node, scope)?;
                if let Value::Number(num) = *expression_value {
                    elements.push_back(Value::Number(num))
                } else {
                    println!("Invalid element type for an array.");
                    return Err(())
                }
            },
            ASTNodeTypes::ArrayLiteral => {
                let array_value =
                    resolve(&node, scope)?;
                elements.push_back(Value::create(array_value));
            },
            _ => {
                println!("Invalid array element.");
                return Err(())
            }
        }
    }

    Ok(elements)
}
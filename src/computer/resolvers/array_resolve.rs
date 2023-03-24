use crate::computer::expression_compute::expression_compute;
use crate::public::run_time::global::Global;
use crate::public::value::value::{Value, ValueVec};
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};

pub fn array_resolve(
    array_node: &ASTNode,
    global: &mut Global,
) -> Result<Value, ()> {
    let params =
        array_node.params
        .as_ref()
        .unwrap();
    let mut elements = ValueVec::new();
    
    for node in params {
        match node.type__ {
            ASTNodeTypes::Expression => {
                let expression_value =
                    expression_compute(&node, global)?;
                elements.push(expression_value);
            },
            ASTNodeTypes::ArrayLiteral => {
                let array_value =
                    array_resolve(&node, global)?;
                elements.push(array_value);
            },
            _ => {
                println!("Invalid array element.");
                return Err(())
            }
        }
    }
    Ok(Value::Array(elements))
}
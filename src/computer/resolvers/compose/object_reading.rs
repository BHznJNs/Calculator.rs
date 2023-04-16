use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::value::value::Value;

pub fn assign(
    obj_value: Value,
    prop_node: &ASTNode,
    value: Value,
) -> Result<(), ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        println!("Invalid object reading.");
        return Err(())
    };
    let ASTNodeTypes::Variable(prop_name) = &prop_node.type__ else {
        println!("Invalid object property name.");
        return Err(())
    };
    let obj =
        obj_ref.as_ref().borrow();
    obj.set(prop_name, value)?;
    Ok(())
}

pub fn resolve(
    obj_value: Value,
    prop_node: &ASTNode,
) -> Result<Value, ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        println!("Invalid object reading.");
        return Err(())
    };

    let ASTNodeTypes::Variable(prop_name) = &prop_node.type__ else {
        println!("Invalid object property name.");
        return Err(())
    };

    let obj = obj_ref.as_ref().borrow();
    let prop_value = obj.get(prop_name)?;
    Ok(prop_value)
}
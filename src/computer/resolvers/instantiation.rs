use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::object::Object;
use crate::public::value::value::Value;

use super::{array_literal, variable_reading};

pub fn resolve(
    node:  &ASTNode,
    scope: &mut Scope,
) -> Result<Object, ()> {
    let ASTNodeTypes::Instantiation(class_name) =
        &node.type__ else {
        println!("Invalid class name for instantiation.");
        return Err(())
    };

    let target_class_value =
        variable_reading::resolve(class_name, scope)?;
    let Value::Class(target_class) =
        target_class_value.as_ref() else {
        println!("'{}' is not a valid class.", class_name);
        return Err(())
    };

    let array_node =
        &node.params
        .as_ref()
        .unwrap()[0];
    let param_arr =
        array_literal::resolve(array_node, scope)?;

    Class::instantiate(
        target_class.clone(),
        param_arr,
    )
}
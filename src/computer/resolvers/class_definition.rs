use crate::public::compile_time::ast::types::ClassDefinitionNode;
use crate::public::value::function::{Function, Overload};
use crate::public::value::oop::class::Class;

use super::function_definition;

pub fn resolve(node: &ClassDefinitionNode) -> Result<Class, ()> {
    let mut method_stack = Vec::<(String, Function)>::new();
    for function_node in &node.method_nodes {
        let function_def = function_definition::resolve(function_node)?;
        method_stack.push((
            function_node.name.clone().unwrap(),
            Function::create(function_def),
        ));
    }
    return Ok(Class::new(
        node.properties.clone(),
        method_stack
    ));
}

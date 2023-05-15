use std::collections::HashMap;

use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::compile_time::ast::types::ClassDefinitionNode;
use crate::public::value::function::{Function, Overload};
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;

use super::function_definition;

pub fn resolve(
    node: ClassDefinitionNode
) -> Result<Class, ()> {
    let mut property_stack = Vec::<String>::new();
    let mut method_stack =
        Vec::<(String, Function)>::new();

    for param in node.params {
        if let ASTNode::Variable(sub_node) = param {
            property_stack.push(sub_node.name)
        } else
        if let ASTNode::FunctionDefinition(mut sub_node) = param {
            let method_name =
                sub_node.name.take();
            let function_definition =
                function_definition::resolve(sub_node)?;
            method_stack.push((
                method_name.unwrap(),
                Function::create(function_definition)
            ))
        } else {
            println!("Unexpected node type in class_resolver.");
            return Err(())
        }
    }

    // --- --- --- --- --- ---

    let storage_pattern =
    if method_stack.len() > Class::STORAGE_THRESHOLD {
        DataStoragePattern::Map
    } else {
        DataStoragePattern::List
    };

    let method_list: Option<Vec<(String, Function)>>;
    let method_map : Option<HashMap<String, Function>>;
    match storage_pattern {
        DataStoragePattern::List => {
            method_list = Some(method_stack);
            method_map  = None;
        },
        DataStoragePattern::Map => {
            let mut temp_map =
                HashMap::<String, Function>::new();
            for (k, v) in method_stack {
                temp_map.insert(k, v);
            }
            method_list = None;
            method_map  = Some(temp_map);
        },
    }

    Ok(Class {
        properties: property_stack,

        method_storage: storage_pattern,
        method_list,
        method_map,
    })
}
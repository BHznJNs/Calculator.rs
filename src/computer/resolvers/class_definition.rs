use std::collections::HashMap;

use crate::public::compile_time::ast::types::ClassDefinitionNode;
use crate::public::value::function::{Function, Overload};
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;

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

    let method_list: Option<Vec<(String, Function)>>;
    let method_map: Option<HashMap<String, Function>>;

    let storage_pattern = if node.method_nodes.len() > Class::STORAGE_THRESHOLD {
        DataStoragePattern::Map
    } else {
        DataStoragePattern::List
    };

    match storage_pattern {
        DataStoragePattern::List => {
            method_list = Some(method_stack);
            method_map = None;
        }
        DataStoragePattern::Map => {
            let mut temp_map = HashMap::<String, Function>::new();
            for (k, v) in method_stack {
                temp_map.insert(k, v);
            }
            method_list = None;
            method_map = Some(temp_map);
        }
    }

    Ok(Class {
        properties: node.properties.clone(),
        method_storage: storage_pattern,
        method_list,
        method_map,
    })
}

use std::collections::HashMap;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::value::function::{Function, Overload};
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;

use super::function_definition;

pub fn resolve(
    node: &ASTNode
) -> Result<Class, ()> {
    let params =
        node.params
        .as_ref()
        .unwrap();

    let mut property_stack = Vec::<String>::new();
    let mut method_stack =
        Vec::<(String, Function)>::new();

    for param in params {
        if let ASTNodeTypes::Variable(name) = &param.type__ {
            if param.params.is_some() {
                // class method
                let func_node =
                    &param.params
                    .as_ref()
                    .unwrap()[0];

                if let ASTNodeTypes::FunctionDefinition(_) =
                  &func_node.type__ {
                    let func_definition =
                        function_definition::resolve(func_node)?;
                    method_stack.push((
                        name.clone(),
                        Function::create(func_definition)
                    ))
                }
            } else {
                // class property
                property_stack.push(name.clone())
            }
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
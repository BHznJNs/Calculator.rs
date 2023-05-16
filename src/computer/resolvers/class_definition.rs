use std::collections::HashMap;
use std::rc::Rc;

use crate::public::compile_time::ast::types::ClassDefinitionNode;
use crate::public::value::function::{Function, Overload};
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;

use super::function_definition;

pub fn resolve(
    node: Rc<ClassDefinitionNode>
) -> Result<Class, ()> {
    let mut method_stack =
        Vec::<(String, Function)>::new();
    for function_node in &node.method_nodes {
        let function_def =
            function_definition::resolve(function_node.clone())?;
        method_stack.push((
            function_node.name.clone().unwrap(),
            Function::create(function_def),
        ));
    }

    let method_list: Option<Vec<(String, Function)>>;
    let method_map : Option<HashMap<String, Function>>;

    let storage_pattern =
    if node.method_nodes.len() > Class::STORAGE_THRESHOLD {
        DataStoragePattern::Map
    } else {
        DataStoragePattern::List
    };

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
        properties: node.properties.clone(),
        method_storage: storage_pattern,
        method_list,
        method_map,
    })
}

// pub fn resolve(
//     node: Rc<ClassDefinitionNode>
// ) -> Result<Class, ()> {
//     let mut property_stack =
//         Vec::<Property>::new();
//     let mut method_stack =
//         Vec::<(String, Function)>::new();
    
//     let params = &node.params;
//     let mut index = 0;

//     while index < params.len() {
//         let current = &params[index];

//         if let ASTNode::Variable(sub_node) = current {
//             let next_node =
//                 params.get(index + 1);
//             // property_stack.push(sub_node.name.clone())
//         } else
//         if let ASTNode::FunctionDefinition(sub_node) = current {
//             let function_definition =
//                 function_definition::resolve(sub_node.clone())?;
//             method_stack.push((
//                 sub_node.name.clone().unwrap(),
//                 Function::create(function_definition)
//             ))
//         } else {
//             println!("Unexpected node type in class_resolver.");
//             return Err(())
//         }
//         index += 1;
//     }

//     // --- --- --- --- --- ---

//     let storage_pattern =
//     if method_stack.len() > Class::STORAGE_THRESHOLD {
//         DataStoragePattern::Map
//     } else {
//         DataStoragePattern::List
//     };

//     let method_list: Option<Vec<(String, Function)>>;
//     let method_map : Option<HashMap<String, Function>>;
//     match storage_pattern {
//         DataStoragePattern::List => {
//             method_list = Some(method_stack);
//             method_map  = None;
//         },
//         DataStoragePattern::Map => {
//             let mut temp_map =
//                 HashMap::<String, Function>::new();
//             for (k, v) in method_stack {
//                 temp_map.insert(k, v);
//             }
//             method_list = None;
//             method_map  = Some(temp_map);
//         },
//     }

//     Ok(Class {
//         properties: property_stack,

//         method_storage: storage_pattern,
//         method_list,
//         method_map,
//     })
// }
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use crate::public::value::function::Function;
use crate::public::value::value::{ArrayLiteral, Value};

use super::object::Object;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq)]
pub struct Class {
    pub properties: Vec<String>,

    pub method_storage: DataStoragePattern,
    pub method_list: Option<Vec<(String, Function)>>,
    pub method_map : Option<HashMap<String, Function>>,
}

impl Class {
    pub const STORAGE_THRESHOLD: usize = 8;

    pub fn get_method(
        &self, target_method: &String
    ) -> Result<Function, ()> {
        let result_target_method =
        getter::<Function>(
            self.method_storage,
            target_method,
            &self.method_list,
            &self.method_map,
        );

        match result_target_method {
            Ok(target_method) =>
                Ok(target_method),
            Err(err_msg) => {
                println!("{}", err_msg);
                Err(())
            },
        }
    }

    pub fn instantiate(
        class_self: Rc<Class>,
        mut values: ArrayLiteral
    ) -> Result<Object, ()> {
        let param_count = values.len();
        let storage_pattern =
        if param_count > Class::STORAGE_THRESHOLD {
            DataStoragePattern::Map
        } else {
            DataStoragePattern::List
        };

        let data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>;
        let data_map : Option<HashMap<String, Rc<RefCell<Value>>>>;
        let properties = &class_self.properties;

        match storage_pattern {
            DataStoragePattern::List => {
                let mut list =
                    Vec::<(String, Rc<RefCell<Value>>)>::new();

                let mut index = 0;
                while index < class_self.properties.len() {
                    let current_prop = &properties[index];

                    let current_value =
                    match values.pop_front() {
                        Some(val) => Rc::new(RefCell::new(val)),
                        None => break,
                    };

                    list.push((current_prop.clone(), current_value));
                    index += 1;
                }

                data_list = Some(list);
                data_map  = None;
            },
            DataStoragePattern::Map => {
                let mut map =
                    HashMap::<String, Rc<RefCell<Value>>>::new();

                let mut index = 0;
                while index < class_self.properties.len() {
                    let current_prop = &properties[index];

                    let current_value =
                    match values.pop_front() {
                        Some(val) => Rc::new(RefCell::new(val)),
                        None => break,
                    };

                    map.insert(current_prop.clone(), current_value);
                    index += 1;
                }

                data_list = None;
                data_map  = Some(map);
            },
        }

        Ok(Object {
            prototype: Some(Value::Class(class_self.clone())),
            storage_pattern,
            data_list,
            data_map,
        })
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{{");
        for prop in &self.properties {
            println!("  {},", prop);
        }
        match self.method_storage {
            DataStoragePattern::List => {
                let list =
                    self.method_list
                    .as_ref()
                    .unwrap();
                for method in list {
                    println!("  {}: <Class-Method>,", method.0);
                }
            },
            DataStoragePattern::Map => {
                let map =
                    self.method_map
                    .as_ref()
                    .unwrap();

                for (key, _) in map {
                    println!("  {}: <Class-Method>,", key);
                }
            },
        }
        write!(f, "}}")
    }
}
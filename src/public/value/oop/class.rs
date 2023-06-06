use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{fmt, io};

use crossterm::style::Stylize;

use crate::public::env::ENV_OPTION;
use crate::public::error::{reference_error, type_error, ReferenceType};
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::Function;
use crate::public::value::value::{Value, ValueType};
use crate::utils::output::print_line;

use super::object::Object;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq)]
pub struct Class {
    pub properties: Vec<Property>,

    pub method_storage: DataStoragePattern,
    pub method_list: Option<Vec<(String, Function)>>,
    pub method_map: Option<HashMap<String, Function>>,
}
#[derive(PartialEq, Clone)]
pub struct Property {
    pub type__: ValueType,
    pub identi: String,
}

impl Class {
    pub const STORAGE_THRESHOLD: usize = 8;

    pub fn get_method(&self, target_method: &str) -> Result<Function, ()> {
        let result_target_method = getter::<Function>(
            self.method_storage,
            target_method,
            &self.method_list,
            &self.method_map,
        );

        match result_target_method {
            Ok(target_method) => Ok(target_method),
            Err(()) => Err(reference_error(ReferenceType::Property, target_method)?),
        }
    }

    pub fn instantiate(class_self: Rc<Class>, mut values: ArrayLiteral) -> Result<Object, ()> {
        let param_count = values.len();
        let storage_pattern = if param_count > Class::STORAGE_THRESHOLD {
            DataStoragePattern::Map
        } else {
            DataStoragePattern::List
        };

        let data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>;
        let data_map: Option<HashMap<String, Rc<RefCell<Value>>>>;
        let properties = &class_self.properties;

        let mut temp_list = Vec::<(String, Rc<RefCell<Value>>)>::new();
        let mut index = 0;
        while index < class_self.properties.len() {
            let current_prop = &properties[index];

            let current_value = match values.pop_front() {
                Some(val) => {
                    // check instantiation param type
                    if !val.check_type(current_prop.type__) {
                        return Err(type_error(
                            Some("class instantiation"),
                            vec![current_prop.type__],
                            val.get_type(),
                        )?);
                    }

                    Rc::new(RefCell::new(val))
                }
                None => break,
            };

            temp_list.push((current_prop.identi.clone(), current_value));
            index += 1;
        }

        match storage_pattern {
            DataStoragePattern::List => {
                data_list = Some(temp_list);
                data_map = None;
            }
            DataStoragePattern::Map => {
                let mut temp_map = HashMap::<String, Rc<RefCell<Value>>>::new();
                temp_map.extend(temp_list);

                data_list = None;
                data_map = Some(temp_map);
            }
        }

        Ok(Object {
            prototype: Some(Value::Class(class_self.clone())),
            storage_pattern,
            data_list,
            data_map,
        })
    }
}

const CLASS_METHOD_DISP_STR: &'static str = "<Class-Method>";
impl fmt::Display for Class {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = io::stdout();

        print_line(&mut stdout, '{');
        for prop in &self.properties {
            // todo: display property indentifier and type
            print_line(
                &mut stdout,
                format!("  {}: {},", prop.identi, prop.type__.to_string().red(),),
            );
        }

        let class_method_disp = if unsafe { ENV_OPTION.support_ansi } {
            CLASS_METHOD_DISP_STR.cyan().to_string()
        } else {
            String::from(CLASS_METHOD_DISP_STR)
        };
        match self.method_storage {
            DataStoragePattern::List => {
                let list = self.method_list.as_ref().unwrap();
                for method in list {
                    print_line(
                        &mut stdout,
                        format!("  {}: {},", method.0, class_method_disp),
                    );
                }
            }
            DataStoragePattern::Map => {
                let map = self.method_map.as_ref().unwrap();

                for (key, _) in map {
                    print_line(&mut stdout, format!("  {}: {},", key, class_method_disp));
                }
            }
        }
        print!("}}");
        Ok(())
    }
}

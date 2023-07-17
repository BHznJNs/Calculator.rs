use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use crossterm::style::Stylize;

use crate::public::env::ENV_OPTION;
use crate::public::error::{reference_error, type_error, ReferenceType};
use crate::public::value::array::ArrayLiteral;
use crate::public::value::display_indent;
use crate::public::value::function::Function;
use crate::public::value::value::{Value, ValueType};
use crate::public::Param;
use crate::utils::completer::Completer;

use super::object::Object;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq)]
pub struct Class {
    properties: Vec<Property>,
    pub completer: Option<Completer>,

    method_storage: DataStoragePattern,
    method_list: Option<Vec<(String, Function)>>,
    method_map: Option<HashMap<String, Function>>,
}
#[derive(PartialEq, Clone)]
pub struct Property(pub ValueType, pub String);
impl Param for Property {
    fn type__(&self) -> ValueType {
        self.0
    }
    fn identi(&self) -> &str {
        &self.1
    }
}

impl Class {
    const STORAGE_THRESHOLD: usize = 8;
    const METHOD_DISP_STR: &'static str = "<Class-Method>";

    pub fn new(properties: Vec<Property>, methods: Vec<(String, Function)>) -> Self {
        // get properties' and methods' names into one `Vec`
        let mut prop_name_vec = vec![];
        for Property(_, identi) in &properties {
            prop_name_vec.push(identi.clone())
        }
        for (k, _) in &methods {
            prop_name_vec.push(k.clone());
        }

        // init method list / map
        let method_storage = if methods.len() > Self::STORAGE_THRESHOLD {
            DataStoragePattern::Map
        } else {
            DataStoragePattern::List
        };

        let method_list: Option<Vec<(String, Function)>>;
        let method_map: Option<HashMap<String, Function>>;

        match method_storage {
            DataStoragePattern::List => {
                method_list = Some(methods);
                method_map = None;
            }
            DataStoragePattern::Map => {
                let mut temp_map = HashMap::new();
                for (k, v) in methods {
                    temp_map.insert(k, v);
                }
                method_list = None;
                method_map = Some(temp_map);
            }
        }

        // init completer
        let mut completer = None;
        if unsafe { ENV_OPTION.is_repl } {
            completer = Some(Completer::from(prop_name_vec));
        }

        return Class {
            properties,
            completer,
            method_storage,
            method_list,
            method_map,
        };
    }

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
                    if !val.check_type(current_prop.type__()) {
                        return Err(type_error(
                            Some("class instantiation"),
                            vec![current_prop.type__()],
                            val.get_type(),
                        )?);
                    }

                    Rc::new(RefCell::new(val))
                }
                None => break,
            };

            temp_list.push((current_prop.identi().to_owned(), current_value));
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

        return Ok(Object {
            prototype: class_self.clone(),
            storage_pattern,
            data_list,
            data_map,
        });
    }

    pub fn display_methods(f: &mut fmt::Formatter<'_>, cls: &Class, level: usize) -> fmt::Result {
        let class_method_disp = if unsafe { ENV_OPTION.support_ansi } {
            Class::METHOD_DISP_STR.cyan().to_string()
        } else {
            String::from(Class::METHOD_DISP_STR)
        };

        match cls.method_storage {
            DataStoragePattern::List => {
                let list = cls.method_list.as_ref().unwrap();
                for method in list {
                    write!(f, "{}{}: {}\r\n", display_indent(level), method.0, class_method_disp)?;
                }
            }
            DataStoragePattern::Map => {
                let map = cls.method_map.as_ref().unwrap();

                for (key, _) in map {
                    write!(f, "{}{}: {}\r\n", display_indent(level), key, class_method_disp)?;
                }
            }
        }
        return Ok(());
    }
}


impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\r\n")?;
        // display class properties
        for prop in &self.properties {
            write!(f, "{}{}: {}\r\n", display_indent(1), prop.identi(), prop.type__().to_string().red())?;
        }

        Class::display_methods(f, self, 1)?;
        write!(f, "}}")
    }
}

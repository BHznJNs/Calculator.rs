use std::fmt;
use std::rc::Rc;

use crossterm::style::Stylize;

use crate::public::env::ENV_OPTION;
use crate::public::error::{reference_error, type_error, ReferenceType};
use crate::public::value::array::ArrayLiteral;
use crate::public::value::display_indent;
use crate::public::value::function::Function;
use crate::public::value::value::{ValueType, Value};
use crate::public::Param;
use crate::utils::completer::Completer;

use super::object::Object;
use super::data_storage::{DataStoragePattern, ComposeStorage, self};

#[derive(PartialEq)]
pub struct Class {
    properties: Vec<Property>,
    method_storage: ComposeStorage<Function>,

    pub completer: Option<Completer>,
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

        // init method storage
        let method_storage = ComposeStorage::new(methods);

        // init completer
        let mut completer = None;
        if unsafe { ENV_OPTION.is_repl } {
            completer = Some(Completer::from(prop_name_vec));
        }

        return Class {
            properties,
            method_storage,
            completer,
        };
    }

    pub fn get_method(&self, method_name: &str) -> Result<Function, ()> {
        let result_method = self.method_storage.getter(method_name);
        match result_method {
            Ok(func) => Ok(func),
            Err(_) => Err(reference_error(ReferenceType::Property, method_name)?),
        }
    }

    pub fn instantiate(class_self: Rc<Class>, mut values: ArrayLiteral) -> Result<Object, ()> {
        let properties = &class_self.properties;
        let mut temp_list = data_storage::ListStorage::<Value>::new();
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
                    val.into()
                }
                None => break,
            };

            temp_list.push((current_prop.identi().to_owned(), current_value));
            index += 1;
        }

        return Ok(Object {
            prototype: class_self.clone(),
            storage: ComposeStorage::new(temp_list),
        });
    }

    pub fn display_methods(f: &mut fmt::Formatter<'_>, cls: &Class, level: usize) -> fmt::Result {
        let class_method_disp = if unsafe { ENV_OPTION.support_ansi } {
            Class::METHOD_DISP_STR.cyan().to_string()
        } else {
            String::from(Class::METHOD_DISP_STR)
        };

        let ComposeStorage {storage_pattern, data_list, data_map} = &cls.method_storage;

        match storage_pattern {
            DataStoragePattern::List => {
                let list = data_list.as_ref().unwrap();
                for method in list {
                    write!(
                        f,
                        "{}{}: {}\r\n",
                        display_indent(level),
                        method.0,
                        class_method_disp
                    )?;
                }
            }
            DataStoragePattern::Map => {
                let map = data_map.as_ref().unwrap();

                for (key, _) in map {
                    write!(
                        f,
                        "{}{}: {}\r\n",
                        display_indent(level),
                        key,
                        class_method_disp
                    )?;
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
            write!(
                f,
                "{}{}: {}\r\n",
                display_indent(1),
                prop.identi(),
                prop.type__().to_string().red()
            )?;
        }

        Class::display_methods(f, self, 1)?;
        write!(f, "}}")
    }
}

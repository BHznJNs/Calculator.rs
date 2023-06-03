use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Stdout, self};
use std::rc::Rc;

use crate::public::value::array;
use crate::utils::output::print_line;

use super::super::value::Value;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq, Clone)]
pub struct Object {
    pub prototype: Option<Value>,

    pub storage_pattern: DataStoragePattern,
    pub data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>,
    pub data_map: Option<HashMap<String, Rc<RefCell<Value>>>>,
}

pub fn display(obj: Rc<RefCell<Object>>, level: usize) {
    fn display_item(stdout: &mut Stdout, key: &String, value: &Rc<RefCell<Value>>, level: usize) {
        let value_ref = value.as_ref().borrow();

        // print indent and key
        print!("{}{}: ", "  ".repeat(level), key);

        // print value
        match value_ref.unwrap() {
            Value::String(_) =>
                print!("{}", value_ref.str_format()),
            Value::Array(arr) =>
                array::display(arr, level + 1),
            Value::Object(obj) =>
                display(obj, level + 1),
            _ => print!("{}", value_ref),
        }

        // next line
        print_line(stdout, "");
    }

    let obj_ref = obj.as_ref().borrow();
    let mut stdout = io::stdout();

    print_line(&mut stdout, '{');
    match obj_ref.storage_pattern {
        DataStoragePattern::List => {
            let list = obj_ref.data_list.as_ref().unwrap();
            for (k, v) in list {
                display_item(&mut stdout, k, v, level);
            }
        }
        DataStoragePattern::Map => {
            let map = obj_ref.data_map.as_ref().unwrap();

            for (k, v) in map {
                display_item(&mut stdout, k, v, level);
            }
        }
    }
    print!("{}}}", "  ".repeat(level - 1));
}

impl Object {
    pub fn get(&self, prop_name: &str) -> Result<Value, ()> {
        let target_value_result = getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );
        match target_value_result {
            Ok(target_rc) => {
                let target_ref = target_rc.as_ref().borrow();
                Ok(target_ref.unwrap())
            }
            Err(_) => match &self.prototype {
                Some(Value::Class(proto)) => {
                    let target_method = proto.get_method(prop_name)?;
                    Ok(Value::Function(target_method.clone()))
                }
                _ => {
                    // todo: replace with reference_error
                    println!("Property '{}' in object does not exist.", prop_name);
                    Err(())
                }
            },
        }
    }

    pub fn set(&self, prop_name: &String, value: Value) -> Result<(), ()> {
        let result_target_rc = getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );

        match result_target_rc {
            Ok(target_rc) => {
                let mut target_ref = target_rc.as_ref().borrow_mut();
                *target_ref = value;
                Ok(())
            }
            Err(err_msg) => {
                // todo: replace with reference_error
                println!("{}", err_msg);
                Err(())
            }
        }
    }
}

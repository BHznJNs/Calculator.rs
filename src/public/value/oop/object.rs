use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Stdout};
use std::rc::Rc;

use crate::public::error::{reference_error, ReferenceType};
use crate::public::value::array::{self, ArrayLiteral};
use crate::public::value::oop::class::Class;
use crate::public::value::value::Overload;
use crate::utils::output::print_line;

use super::super::value::Value;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq, Clone)]
pub struct Object {
    pub prototype: Rc<Class>,

    pub storage_pattern: DataStoragePattern,
    pub data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>,
    pub data_map: Option<HashMap<String, Rc<RefCell<Value>>>>,
}

pub fn deep_clone(obj: Rc<RefCell<Object>>) -> Value {
    fn prop_value_resolve(v: &Rc<RefCell<Value>>, param_vec: &mut ArrayLiteral) {
        let v_ref = v.as_ref().borrow();
        if let Value::Object(sub_obj) = v_ref.unwrap() {
            param_vec.push_back(deep_clone(sub_obj));
        } else {
            param_vec.push_back(v_ref.deep_clone());
        }
    }

    let obj_ref = &*(obj.as_ref().borrow());
    let mut instantiation_params = ArrayLiteral::new();

    match obj_ref.storage_pattern {
        DataStoragePattern::List => {
            if let Some(list) = &obj_ref.data_list {
                for (_, v) in list {
                    prop_value_resolve(v, &mut instantiation_params);
                }
            }
        }
        DataStoragePattern::Map => {
            if let Some(map) = &obj_ref.data_map {
                for (_, v) in map {
                    prop_value_resolve(v, &mut instantiation_params);
                }
            }
        }
    }

    // the object has passed the type check,
    // thus with properties of the object,
    // the instantiation must pass the type check.
    let res_object = Class::instantiate(
        obj_ref.prototype.clone(),
        instantiation_params
    ).unwrap();
    Value::create(res_object)
}

pub fn display(obj: Rc<RefCell<Object>>, level: usize) {
    fn display_item(stdout: &mut Stdout, key: &String, value: &Rc<RefCell<Value>>, level: usize) {
        let value_ref = value.as_ref().borrow();

        // print indent and key
        print!("{}{}: ", "  ".repeat(level), key);

        // print value
        match value_ref.unwrap() {
            Value::String(_) => print!("{}", value_ref.str_format()),
            Value::Array(arr) => array::display(arr, level + 1),
            Value::Object(obj) => display(obj, level + 1),
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
            Err(_) => {
                let target_method = self.prototype.get_method(prop_name)?;
                Ok(Value::Function(target_method.clone()))
            }
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
            Err(()) => Err(reference_error(ReferenceType::Property, prop_name)?),
        }
    }
}

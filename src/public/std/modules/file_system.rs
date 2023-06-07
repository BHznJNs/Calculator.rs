use std::path::Path;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_val::get_val;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFunction, Param, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{Value, ValueType, Overload as ValueOverload};

use super::BuildInFnCall;

#[derive(PartialEq)]
pub enum FileSysFn {
    Open,
    Read,
    Write,
}

static mut FILE_CLASS: Option<Rc<Class>> = None;

fn static_class_setter() {
    let read = BuildInFunction {
        params: vec![Param {
            type__: ValueType::Object,
            identi: "self",
        }],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Open),
    };

    unsafe {
        FILE_CLASS = Some(Rc::new(Class {
            properties: vec![
                Property {
                    type__: ValueType::String,
                    identi: String::from("path"),
                },
                Property {
                    type__: ValueType::Boolean,
                    identi: String::from("exist"),
                },
                Property {
                    type__: ValueType::Boolean,
                    identi: String::from("is_dir"),
                },
                Property {
                    type__: ValueType::Boolean,
                    identi: String::from("is_file"),
                },
            ],
            method_storage: DataStoragePattern::List,
            method_list: Some(vec![
                (String::from("read"), Function::BuildIn(read.into()))
            ]),
            method_map: None,
        }))
    };
}

pub fn module_object() -> Object {
    static_class_setter();

    let open = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::String,
                identi: "path",
            },
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Open),
    };

    // --- --- --- --- --- ---

    Object {
        prototype: None,
        storage_pattern: DataStoragePattern::List,
        data_list: Some(vec![
            (String::from("open"), Value::create(open).into())
        ]),
        data_map: None,
    }
}

impl BuildInFnCall for FileSysFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            FileSysFn::Open => {
                let path_value = get_val("path", scope)?;

                let Value::String(str) = path_value.clone() else {
                    unreachable!() 
                };

                let str_ref = str.borrow();
                let str_slice = str_ref.as_str();

                let path = Path::new(str_slice);
                let path_exist = path.exists();
                let path_is_dir = path.is_dir();
                let path_is_file = path.is_file();

                let file_obj =
                    unsafe { Class::instantiate(
                        FILE_CLASS.as_ref().unwrap().clone(),
                        ArrayLiteral::from([
                            path_value,
                            Value::Boolean(path_exist),
                            Value::Boolean(path_is_dir),
                            Value::Boolean(path_is_file),
                        ])
                    )? };
                
                
                Value::create(file_obj)
            }
            FileSysFn::Read => todo!(),
            FileSysFn::Write => todo!(),
        };
        Ok(result)
    }
}

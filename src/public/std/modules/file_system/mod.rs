mod file_ops;

use std::path::Path;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::modules::file_system::file_ops::{file_read, file_write};
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::std::utils::get_val::get_val;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFunction, Param, Function, Overload};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{Value, ValueType, Overload as ValueOverload, VoidSign};

use self::file_ops::file_append;

use super::BuildInFnCall;

#[derive(PartialEq)]
pub enum FileSysFn {
    Open,
    Read,
    Write,
    Append,
}

static mut FILE_CLASS: Option<Rc<Class>> = None;

fn static_class_setter() {
    let read = BuildInFunction {
        params: vec![Param {
            type__: ValueType::Object,
            identi: "self",
        }],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Read),
    };

    let write = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::String,
                identi: "content",
            },
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Write),
    };
    let append = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::String,
                identi: "content",
            },
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Append),
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
                (String::from("read"), Function::create(read)),
                (String::from("write"), Function::create(write)),
                (String::from("append"), Function::create(append)),
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
            (String::from("open"), Value::create(open).into()),
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
            _ => {
                // the code following is used as the method of class `File`.
                let self_value = get_val("self", scope)?;

                let self_path = get_self_prop(&self_value, "path")?;
                let exist = get_self_prop(&self_value, "exist")?;
                let is_dir = get_self_prop(&self_value, "is_dir")?;
                let is_file = get_self_prop(&self_value, "is_file")?;
                let file_info = (exist, is_dir, is_file);

                // these 5 lines is to get the `&str` typed path data.
                let Value::String(path_str) = self_path else {
                    unreachable!()
                };
                let path_str_temp = path_str.borrow();
                let file_path = path_str_temp.as_str();

                match self {
                    FileSysFn::Read =>
                        file_read(file_path, file_info)?,
                    FileSysFn::Write => {
                        let content_value = get_val("content", scope)?;
                        file_write(file_path, content_value, file_info)?;
                        Value::Void(VoidSign::Empty)
                    },
                    FileSysFn::Append => {
                        let content_value = get_val("content", scope)?;
                        file_append(file_path, content_value, file_info)?;
                        Value::Void(VoidSign::Empty)
                    },
                    _ => unreachable!()
                }
            }
        };
        Ok(result)
    }
}

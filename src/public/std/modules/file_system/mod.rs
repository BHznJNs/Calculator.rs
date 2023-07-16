mod file_ops;
mod fs_ops;

use std::path::Path;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::modules::file_system::file_ops::{file_read, file_write};
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::std::utils::get_val::get_val;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function, Overload};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::object::Object;
use crate::public::value::value::{Overload as ValueOverload, Value, ValueType, VoidSign};

use self::file_ops::file_append;
use self::fs_ops::{dir_create, dir_delete, file_create, file_delete};

use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
pub enum FileSysFn {
    Open,
    Create,
    Delete,

    Read,
    Write,
    Append,
}

static mut MODULE_CLASS: Option<Rc<Class>> = None;
static mut FILE_CLASS: Option<Rc<Class>> = None;

fn static_class_setter() {
    // file-class methods
    let read = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Read),
    };
    let write = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "content"),
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Write),
    };
    let append = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "content"),
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Append),
    };

    // fs-class methods
    let fs_method_template = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "path"),
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysFn::Open),
    };
    let open = fs_method_template.clone();
    let mut create = fs_method_template.clone();
    let mut delete = fs_method_template.clone();
    create.identi = BuildInFnIdenti::FileSystem(FileSysFn::Create);
    delete.identi = BuildInFnIdenti::FileSystem(FileSysFn::Delete);

    // --- --- --- --- --- ---

    unsafe {
        FILE_CLASS = Some(
            Class::new(
                vec![
                    Property(ValueType::String, String::from("path")),
                    Property(ValueType::String, String::from("exist")),
                    Property(ValueType::Boolean, String::from("is_dir")),
                    Property(ValueType::Boolean, String::from("is_file")),
                ],
                vec![
                    (String::from("read"), Function::create(read)),
                    (String::from("write"), Function::create(write)),
                    (String::from("append"), Function::create(append)),
                ],
            )
            .into(),
        );
        // --- --- --- --- --- ---
        MODULE_CLASS = Some(
            Class::new(
                vec![],
                vec![
                    (String::from("open"), Function::create(open)),
                    (String::from("create"), Function::create(create)),
                    (String::from("delete"), Function::create(delete)),
                ],
            )
            .into(),
        );
    };
}

pub fn module_object() -> Object {
    if unsafe { MODULE_CLASS == None || FILE_CLASS == None } {
        static_class_setter();
    }

    return Class::instantiate(
        unsafe { MODULE_CLASS.as_ref().unwrap().clone() },
        ArrayLiteral::new(),
    )
    .unwrap();
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

                let file_obj = unsafe {
                    Class::instantiate(
                        FILE_CLASS.as_ref().unwrap().clone(),
                        ArrayLiteral::from([
                            path_value,
                            Value::Boolean(path_exist),
                            Value::Boolean(path_is_dir),
                            Value::Boolean(path_is_file),
                        ]),
                    )?
                };

                Value::create(file_obj)
            }
            FileSysFn::Create => {
                let path_value = get_val("path", scope)?;
                let Value::String(str) = path_value.clone() else {
                    unreachable!()
                };

                let path_ref = str.borrow();
                let path_str = path_ref.as_str();
                if path_str.ends_with('/') || path_str.ends_with('\\') {
                    dir_create(path_str)?;
                } else {
                    file_create(path_str)?;
                }

                FileSysFn::Open.call(scope)?
            }
            FileSysFn::Delete => {
                let path_value = get_val("path", scope)?;
                let Value::String(str) = path_value.clone() else {
                    unreachable!()
                };

                let path_ref = str.borrow();
                let path_str = path_ref.as_str();
                let path = Path::new(path_str);
                if path.is_dir() {
                    dir_delete(path_str)?;
                } else {
                    file_delete(path_str)?;
                }

                Value::Void(VoidSign::Empty)
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
                    FileSysFn::Read => file_read(file_path, file_info)?,
                    FileSysFn::Write => {
                        let content_value = get_val("content", scope)?;
                        file_write(file_path, content_value, file_info)?;
                        Value::Void(VoidSign::Empty)
                    }
                    FileSysFn::Append => {
                        let content_value = get_val("content", scope)?;
                        file_append(file_path, content_value, file_info)?;
                        Value::Void(VoidSign::Empty)
                    }
                    _ => unreachable!(),
                }
            }
        };
        return Ok(result);
    }
}
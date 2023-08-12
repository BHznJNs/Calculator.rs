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
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::object::Object;
use crate::public::value::value::{Value, ValueType};

use self::file_ops::file_append;
use self::fs_ops::{dir_create, dir_delete, file_create, file_delete};

use super::{BuildInFnCall, ObjectModule};

#[derive(PartialEq, Clone)]
pub enum FileSysModule {
    Open,
    Create,
    Delete,

    Read,
    Write,
    Append,
}

static mut FILE_CLASS: Option<Rc<Class>> = None;

fn static_class_setter() {
    // file-class methods
    let read = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Object, "self")],
        identi: BuildInFnIdenti::FileSystem(FileSysModule::Read),
    };
    let write = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "content"),
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysModule::Write),
    };
    let append = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "content"),
        ],
        identi: BuildInFnIdenti::FileSystem(FileSysModule::Append),
    };
    // --- --- --- --- --- ---
    unsafe {
        FILE_CLASS = Some(
            Class::new(
                vec![
                    Property(ValueType::String, String::from("path")),
                    Property(ValueType::Boolean, String::from("exist")),
                    Property(ValueType::Boolean, String::from("is_dir")),
                    Property(ValueType::Boolean, String::from("is_file")),
                ],
                vec![
                    (String::from("read"), Function::from(read)),
                    (String::from("write"), Function::from(write)),
                    (String::from("append"), Function::from(append)),
                ],
            )
            .into(),
        );
    };
}

impl ObjectModule for FileSysModule {
    fn module_object() -> Object {
        if unsafe { FILE_CLASS == None } {
            static_class_setter();
        }

        // fs-class methods
        let fs_method_template = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::String, "path"),
            ],
            identi: BuildInFnIdenti::FileSystem(Self::Open),
        };
        let open = fs_method_template.clone();
        let mut create = fs_method_template.clone();
        let mut delete = fs_method_template.clone();
        create.identi = BuildInFnIdenti::FileSystem(Self::Create);
        delete.identi = BuildInFnIdenti::FileSystem(Self::Delete);

        let module_obj_props = vec![
            (String::from("open"), Value::from(open)),
            (String::from("create"), Value::from(create)),
            (String::from("delete"), Value::from(delete)),
        ];
        return Object::new(module_obj_props, None);
    }
}

impl BuildInFnCall for FileSysModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            Self::Open | Self::Create | Self::Delete => {
                let path_value = get_val("path", scope)?;
                let temp = path_value.clone();
                let temp = temp.get_str()?;
                let path_str = temp.as_str();

                match self {
                    Self::Open => {
                        let path = Path::new(path_str);
                        let path_exist = path.exists();
                        let path_is_dir = path.is_dir();
                        let path_is_file = path.is_file();

                        let file_obj = unsafe {
                            Class::instantiate(
                                FILE_CLASS.as_ref().unwrap().clone(),
                                ArrayLiteral::from([
                                    path_value,
                                    Value::from(path_exist),
                                    Value::from(path_is_dir),
                                    Value::from(path_is_file),
                                ]),
                            )?
                        };

                        Value::from(file_obj)
                    }
                    Self::Create => {
                        if path_str.ends_with('/') || path_str.ends_with('\\') {
                            dir_create(path_str)?;
                        } else {
                            file_create(path_str)?;
                        }
                        Value::EMPTY
                    }
                    Self::Delete => {
                        let path = Path::new(path_str);
                        if path.is_dir() {
                            dir_delete(path_str)?;
                        } else {
                            file_delete(path_str)?;
                        }
                        Value::EMPTY
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                // the code following is used as the method of class `File`.
                let self_value = get_val("self", scope)?;

                let self_path = get_self_prop(&self_value, "path")?;
                let exist = get_self_prop(&self_value, "exist")?;
                let is_dir = get_self_prop(&self_value, "is_dir")?;
                let is_file = get_self_prop(&self_value, "is_file")?;
                let file_info = (exist, is_dir, is_file);

                let temp = self_path.get_str()?;
                let file_path = temp.as_str();

                match self {
                    Self::Read => file_read(file_path, file_info)?,
                    Self::Write => {
                        let content_value = get_val("content", scope)?;
                        file_write(file_path, content_value, file_info)?;
                        Value::EMPTY
                    }
                    Self::Append => {
                        let content_value = get_val("content", scope)?;
                        file_append(file_path, content_value, file_info)?;
                        Value::EMPTY
                    }
                    _ => unreachable!(),
                }
            }
        };
        return Ok(result);
    }
}

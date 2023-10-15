pub mod array;
pub mod basic;
pub mod bit_ops;
pub mod file_system;
pub mod map;
pub mod math;
pub mod string;

use std::rc::Rc;

use crate::public::error::CalcResult;
use crate::public::run_time::scope::Scope;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::object::Object;
use crate::public::value::Value;

use super::StdModules;

pub fn import_resolver(scope: &mut Scope, target_module: &StdModules, module_name: &str) {
    match target_module {
        StdModules::Basic | StdModules::BitOps => {
            let fn_list = target_module.get_fn_list();
            if let Some(completer) = &mut scope.completer {
                for (k, _) in &fn_list {
                    completer.insert(k);
                }
            }

            scope.global.variables.extend(fn_list);
        }

        StdModules::FileSystem | StdModules::Math => {
            let module_obj = target_module.get_obj_entry();
            scope.assign(String::from(module_name), Value::from(module_obj));
        }

        StdModules::String | StdModules::Array | StdModules::Map => {
            let module_cls = target_module.get_cls_entry();
            scope.assign(String::from(module_name), Value::Class(module_cls));
        }
    }
}

pub trait BuildInFnCall {
    fn call(&self, scope: &mut Scope) -> CalcResult<Value>;
}

pub trait FunctionModule: BuildInFnCall {
    fn function_list() -> Vec<(String, Value)>;
}

pub trait ClassModule: BuildInFnCall {
    fn __static_class__() -> Class;
    fn module_class() -> Rc<Class>;
}
pub trait ObjectModule: BuildInFnCall {
    fn module_object() -> Object;
}

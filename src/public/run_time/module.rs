use crate::public::std::StdModules;
use crate::public::std::modules::{basic, math, array, string};
use crate::public::value::value::{Value, Overload};

use super::scope::Scope;

pub fn resolve(
    target_module: &StdModules,
    scope: &mut Scope,
) {
    match target_module {
        StdModules::Basic => {
            let fn_list =
                basic::function_list();
            scope.global.variables.extend(fn_list);
        },
        StdModules::Math => {
            let module_obj = math::module_object();
            scope.global.variables.insert(
                String::from("Math"),
                Value::create(module_obj),
            );
        },
        StdModules::Array => {
            let module_cls = array::module_class();
            scope.global.variables.insert(
                String::from("Array"),
                Value::create(module_cls),
            );
        },
        StdModules::String => {
            let module_cls = string::module_class();
            scope.global.variables.insert(
                String::from("String"),
                Value::create(module_cls),
            );
        },
        StdModules::FileSystem => todo!(),
    }
}
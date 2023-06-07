use crate::public::std::modules::basic;
use crate::public::std::StdModules;
use crate::public::value::value::{Overload, Value};

use super::scope::Scope;

pub fn std_resolve(scope: &mut Scope, target_module: &StdModules, module_name: &str) {
    match target_module {
        StdModules::Basic => {
            let fn_list = basic::function_list();
            scope.global.variables.extend(fn_list);
        }

        StdModules::FileSystem
        | StdModules::Math => {
            let module_obj = target_module.get_obj_entry();
            scope.assign(
                String::from(module_name),
                Value::create(module_obj),
            );
        }

        StdModules::String
        | StdModules::Array => {
            let module_cls = target_module.get_cls_entry();
            scope.assign(
                String::from(module_name),
                Value::create(module_cls),
            );
        }
    }
}

use crate::public::run_time::scope::GlobalScope;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::Value;

use super::object::Object;

// convert module scope to Object
pub fn module_create(module_scope: GlobalScope) -> Object {
    let mut val_stack = ArrayLiteral::new();
    let mut prop_stack = vec![];
    let mut method_statck = vec![];

    for (k, v) in module_scope.variables {
        if let Value::Function(func) = v {
            method_statck.push((k, func))
        } else {
            prop_stack.push(Property(v.get_type(), k));
            val_stack.push_back(v);
        }
    }
    let module_class = Class::new(
        prop_stack,
        method_statck,
    );
    return Class::instantiate(
        module_class.into(),
        val_stack,
    ).unwrap();
}

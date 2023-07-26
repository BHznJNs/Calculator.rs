use crate::public::compile_time::ast::types::InstantiationNode;
use crate::public::error::type_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::object::Object;
use crate::public::value::value::{Value, ValueType};

use super::array_literal;

pub fn resolve(node: &InstantiationNode, scope: &mut Scope) -> Result<Object, ()> {
    let target_class_value = scope.read_var(&node.class)?;
    let Value::Class(target_class) =
        target_class_value else {
        return Err(type_error(
            Some("instantiation"),
            vec![ValueType::Class],
            target_class_value.get_type()
        )?)
    };

    let instantiation_params = array_literal::resolve(&node.params, scope)?;
    return Class::instantiate(target_class.clone(), instantiation_params);
}

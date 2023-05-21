use crate::public::{value::{value::Value, oop::object::Object}, error::{internal_error, InternalComponent}};

pub fn get_self_prop(
    self_value: Value,
    prop_name: &str,
) -> Result<Value, ()> {
    let Value::Object(obj) = self_value else {
        return Err(internal_error(
            InternalComponent::Std,
            "invalid object getter invocation for invalid value type"
        )?)
    };

    let obj_ref = obj.as_ref().borrow();
    Object::get(&obj_ref, &prop_name)
}
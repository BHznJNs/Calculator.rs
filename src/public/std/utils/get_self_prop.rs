use crate::public::{
    error::{internal_error, InternalComponent},
    value::value::Value,
};

pub fn get_self_prop(self_value: &Value, prop_name: &str) -> Result<Value, ()> {
    let Value::Object(obj) = self_value else {
        return Err(internal_error(
            InternalComponent::Std,
            "invalid object getter invocation for invalid value type"
        )?)
    };

    let obj_ref = obj.as_ref().borrow();
    return obj_ref.get(prop_name);
}

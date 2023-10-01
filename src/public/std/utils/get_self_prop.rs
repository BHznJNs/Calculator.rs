use crate::public::{
    error::{internal_error, InternalComponent},
    value::Value,
};

pub fn get_self_prop(self_value: &Value, prop_name: &str) -> Result<Value, ()> {
    let Value::Object(obj) = self_value else {
        return Err(internal_error(
            InternalComponent::Std,
            "invalid value type for object getter invocation"
        )?)
    };

    let obj_ref = obj.as_ref().borrow();
    return obj_ref.get(prop_name);
}

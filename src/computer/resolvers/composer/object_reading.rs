use crate::public::{error::syntax_error, value::Value};

pub fn assign(obj_value: Value, property: &str, value: Value) -> Result<(), ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        return Err(syntax_error("invalid object reading")?)
    };

    let mut obj = obj_ref.as_ref().borrow_mut();
    obj.set(property, value)?;
    Ok(())
}

pub fn resolve(obj_value: Value, property: &str) -> Result<Value, ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        return Err(syntax_error("invalid object reading")?)
    };

    let obj = obj_ref.as_ref().borrow();
    let prop_value = obj.get(property)?;
    Ok(prop_value)
}

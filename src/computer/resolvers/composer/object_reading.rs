use crate::public::{error::{syntax_error, CalcResult}, value::Value};

pub fn assign(obj_value: Value, property: &str, value: Value) -> CalcResult<()> {
    let Value::Object(obj_ref) =
        obj_value else {
        return Err(syntax_error("invalid object reading"));
    };

    let mut obj = obj_ref.as_ref().borrow_mut();
    obj.set(property, value)?;
    return Ok(());
}

pub fn resolve(obj_value: Value, property: &str) -> CalcResult<Value> {
    let Value::Object(obj_ref) =
        obj_value else {
        return Err(syntax_error("invalid object reading"))
    };

    let obj = obj_ref.as_ref().borrow();
    let prop_value = obj.get(property)?;
    return Ok(prop_value);
}

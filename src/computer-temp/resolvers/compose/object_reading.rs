use crate::public::value::value::Value;

pub fn assign(
    obj_value: Value,
    property: &String,
    value: Value,
) -> Result<(), ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        println!("Invalid object reading.");
        return Err(())
    };

    let obj =
        obj_ref.as_ref().borrow();
    obj.set(property, value)?;
    Ok(())
}

pub fn resolve(
    obj_value: Value,
    property: &String,
) -> Result<Value, ()> {
    let Value::Object(obj_ref) =
        obj_value else {
        println!("Invalid object reading.");
        return Err(())
    };

    let obj =
        obj_ref.as_ref().borrow();
    let prop_value =
        obj.get(property)?;
    Ok(prop_value)
}
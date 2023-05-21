use crate::public::value::{oop::object::Object, value::Value};

pub fn get_self_prop(self_value: Value, prop_name: &str) -> Result<Value, ()> {
    let Value::Object(obj) = self_value else {
        println!("Invalid array getter invocation.");
        return Err(())
    };

    let obj_ref = obj.as_ref().borrow();
    Object::get(&obj_ref, &prop_name)
}

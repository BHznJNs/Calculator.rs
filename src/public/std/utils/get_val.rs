use std::rc::Rc;

use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

// used to get value of function actual param.
pub fn get_val(
    val_name: &str,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let val =
        scope.local
        .as_ref().unwrap()
        .variables
        .get(val_name);
    match val {
        Some(rc_val) =>
            Ok(rc_val.clone()),
        None => {
            println!("Input for function is missing.");
            Err(())
        },
    }
}
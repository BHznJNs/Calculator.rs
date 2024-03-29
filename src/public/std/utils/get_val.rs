use crate::public::error::syntax_error;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

// used to get value of function actual param.
pub fn get_val(val_name: &str, scope: &mut Scope) -> Result<Value, ()> {
    let option_value = scope.local.as_ref().unwrap().variables.get(val_name);
    match option_value {
        Some(val) => Ok(val.clone()),
        None => {
            let msg = format!("build-in function param '{}' is missing", val_name);
            Err(syntax_error(&msg)?)
        }
    }
}

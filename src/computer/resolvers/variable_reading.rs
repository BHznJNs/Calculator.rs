use crate::public::error::{reference_error, ReferenceType};
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn resolve(var_name: &String, scope: &mut Scope) -> Result<Value, ()> {
    // use local-scope preferer
    if let Some(local_scope) = &scope.local {
        if let Some(val) = local_scope.variables.get(var_name) {
            return Ok(val.clone());
        }
    };

    match scope.global.variables.get(var_name) {
        Some(val) => Ok(val.clone()),
        None => Err(reference_error(ReferenceType::Variable, var_name)?),
    }
}

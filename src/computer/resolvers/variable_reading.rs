use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn resolve(
    var_name: &String,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let variable_value = match &scope.local {
        // use local-scope preferer
        Some(local_scope) =>
            match local_scope.variables.get(var_name) {
                Some(val) => val.clone(),
                None =>
                    match scope.global.variables.get(var_name) {
                        Some(val) => val.clone(),
                        None => {
                            println!("Variable '{}' is undefined.", var_name);
                            return Err(())
                        },
                    },
            },
        None =>
            match scope.global.variables.get(var_name) {
                Some(val) => val.clone(),
                None => {
                    println!("Variable '{}' is undefined.", var_name);
                    return Err(())
                },
            },
    };

    Ok(variable_value)
}
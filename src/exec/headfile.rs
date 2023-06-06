use std::collections::VecDeque;

use crate::public::run_time::{build_in, scope::Scope};

use super::script;

pub fn resolve(headfiles: VecDeque<String>, scope: &mut Scope) {
    let mut headfile_scope = Scope::new(&scope);

    for path in headfiles {
        match script::run(path, &mut headfile_scope) {
            Ok(_) => {
                let headfile_vars = headfile_scope.global.variables;

                // init headfile_scope.global.variables
                headfile_scope.global.variables = build_in::constants();
                // insert variables into global scope
                scope.global.variables.extend(headfile_vars);
            }
            Err(_) => break,
        }
    }
}

use std::collections::VecDeque;

use crate::public::run_time::{constants, scope::Scope};

use super::script;

pub fn resolve(headfiles: &VecDeque<String>, scope: &mut Scope) {
    let mut headfile_scope = Scope::new_from(scope);

    for path in headfiles {
        script::run_with_path(path, &mut headfile_scope);
        let headfile_vars = headfile_scope.global.variables;

        // init headfile_scope.global.variables
        headfile_scope.global.variables = unsafe { constants::entry() };
        // insert variables into global scope
        scope.global.variables.extend(headfile_vars);
    }
}

use std::collections::HashMap;
use std::rc::Rc;

use crate::exec::script;
use crate::public::error::{import_error, reference_error, ReferenceType};
use crate::public::std::StdModules;
use crate::public::value::oop::module::module_create;
use crate::public::value::value::{Overload, VoidSign};
use crate::utils::completer::Completer;

use super::super::value::value::Value;
use super::{build_in, module};

pub struct GlobalScope {
    pub variables: HashMap<String, Value>,
}
impl GlobalScope {
    pub fn init() -> GlobalScope {
        GlobalScope {
            variables: build_in::constants(),
        }
    }
}

pub struct LocalScope {
    pub variables: HashMap<String, Value>,
}
impl LocalScope {
    pub fn init() -> LocalScope {
        LocalScope {
            variables: HashMap::<String, Value>::new(),
        }
    }
}

// --- --- --- --- --- ---

pub struct Scope {
    pub global: GlobalScope,
    pub local: Option<LocalScope>,
    pub completer: Option<Completer>,
    user_module_map: HashMap<String, bool>,
    std_module_map: Rc<HashMap<&'static str, StdModules>>,
}
const STD_MODULE_DATA: [(&'static str, StdModules); 5] = [
    ("Basic", StdModules::Basic),
    ("Math", StdModules::Math),
    ("Array", StdModules::Array),
    ("String", StdModules::String),
    ("FS", StdModules::FileSystem),
];
impl Scope {
    pub fn init() -> Scope {
        Scope {
            global: GlobalScope::init(),
            local: None,
            completer: None,

            user_module_map: HashMap::<String, bool>::new(),
            std_module_map: Rc::new(HashMap::from(STD_MODULE_DATA)),
        }
    }
    // inherit self to create new scope
    pub fn new(&self) -> Scope {
        Scope {
            global: GlobalScope::init(),
            local: None,
            completer: None,
            user_module_map: HashMap::<String, bool>::new(),
            std_module_map: self.std_module_map.clone(),
        }
    }

    pub fn assign(&mut self, var_name: String, value: Value) {
        // if local-scope, assigning variable to
        // the local-scope is preferred.
        match &mut self.local {
            Some(local_scope) => {
                // usually in a function invocation.
                local_scope.variables.insert(var_name, value)
            }
            None => {
                if let Some(completer) = &mut self.completer {
                    completer.insert(&var_name);
                }
                self.global.variables.insert(var_name, value)
            }
        };
    }
    pub fn read_var(&self, var_name: &str) -> Result<Value, ()> {
        // use local-scope preferer
        if let Some(local_scope) = &self.local {
            if let Some(val) = local_scope.variables.get(var_name) {
                return Ok(val.clone());
            }
        };

        match self.global.variables.get(var_name) {
            Some(val) => Ok(val.clone()),
            None => Err(reference_error(ReferenceType::Variable, var_name)?),
        }
    }

    // import standard module
    pub fn import_std(&mut self, module_name: &str) -> Result<(), ()> {
        let std_module_map = self.std_module_map.clone();
        let Some(target_module) =
            std_module_map.get(module_name) else {
            let msg = format!("standard module '{}' does not exist", module_name);
            return Err(import_error(&msg)?)
        };

        module::std_resolve(self, target_module, module_name);
        Ok(())
    }
    // import user defined module
    pub fn import_from_path(&mut self, module_path: &str) -> Result<Value, ()> {
        let mut module_scope = self.new();

        // if module has not been imported
        if self.user_module_map.get(module_path) == None {
            // execute the module file
            script::run(module_path, &mut module_scope)?;

            // import modules that imported by module
            for (module, _) in module_scope.user_module_map {
                if let Some(_) = self.std_module_map.get(module.as_str()) {
                    self.import_std(&module)?;
                } else {
                    self.import_from_path(&module)?;
                }
            }
            // identify this path as imported
            self.user_module_map.insert(String::from(module_path), true);
            let module_obj = module_create(module_scope.global);
            Ok(Value::create(module_obj))
        } else {
            Ok(Value::Void(VoidSign::Empty))
        }
    }
}

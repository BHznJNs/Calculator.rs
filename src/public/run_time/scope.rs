use std::collections::HashMap;
use std::rc::Rc;

use crate::exec::script;
use crate::public::error::{import_error, reference_error, ReferenceType};
use crate::public::std::StdModules;
use crate::public::value::oop::module::{get_module_name, module_create};
use crate::public::value::value::Overload;

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
    module: HashMap<String, bool>,
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
            module: HashMap::<String, bool>::new(),
            std_module_map: Rc::new(HashMap::from(STD_MODULE_DATA)),
        }
    }
    // inherit self to create new scope
    pub fn new(&self) -> Scope {
        Scope {
            global: GlobalScope::init(),
            local: None,
            module: HashMap::<String, bool>::new(),
            std_module_map: self.std_module_map.clone(),
        }
    }

    pub fn assign(&mut self, var_name: String, value: Value) {
        // if local-scope, assigning variable to
        // the local-scope is preferred.
        match &mut self.local {
            Some(local_scope) =>
            // usually in a function invocation.
                local_scope
                    .variables
                    .insert(var_name, value),
            None => self
                .global
                .variables
                .insert(var_name, value),
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

    // import STD module
    pub fn import_std(&mut self, module_name: &str) -> Result<(), ()> {
        let std_module_map = self.std_module_map.clone();
        let Some(target_module) =
            std_module_map.get(module_name) else {
            let msg = format!("standard module '{}' does not exist", module_name);
            return Err(import_error(&msg)?)
        };

        if let None = self.module.get(module_name) {
            self.module.insert(module_name.to_string(), true);
            module::resolve(target_module, self);
        }
        Ok(())
    }
    pub fn import_from_path(&mut self, module_path: &str) -> Result<(), ()> {
        let mut module_scope = self.new();
        let module_name = get_module_name(module_path);

        if let None = self.module.get(module_name) {
            // execute the module file
            script::run(module_path.to_string(), &mut module_scope)?;

            // import modules that imported by module
            for (module, _) in module_scope.module {
                if let Some(_) = self.std_module_map.get(module.as_str()) {
                    self.import_std(&module)?;
                } else {
                    self.import_from_path(&module)?;
                }
            }

            // regard the whole module as an Object
            let module_obj = module_create(module_scope.global);
            // insert the Object as a variable into
            // the global scope.
            self.global
                .variables
                .insert(module_name.to_string(), Value::create(module_obj));

            self.module.insert(module_name.to_string(), true);
        }

        Ok(())
    }
}

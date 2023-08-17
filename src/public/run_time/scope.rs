use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::exec::script;
use crate::public::error::{import_error, reference_error, ReferenceType};
use crate::public::std::StdModules;
use crate::public::value::oop::module::module_create;
use crate::utils::completer::Completer;

use super::super::std::modules;
use super::super::value::value::Value;
use super::constants;

pub struct GlobalScope {
    pub variables: HashMap<String, Value>,
}
impl GlobalScope {
    pub fn init() -> Self {
        Self {
            variables: unsafe { constants::entry() },
        }
    }
}

pub struct LocalScope {
    pub variables: HashMap<String, Value>,
}
impl LocalScope {
    pub fn init() -> Self {
        Self {
            variables: HashMap::<String, Value>::new(),
        }
    }
}

// --- --- --- --- --- ---

const STD_MODULE_COUNT: usize = 7;
const STD_MODULE_DATA: [(&'static str, StdModules); STD_MODULE_COUNT] = [
    ("Basic", StdModules::Basic),
    ("Math", StdModules::Math),
    ("Array", StdModules::Array),
    ("String", StdModules::String),
    ("Map", StdModules::Map),
    ("FS", StdModules::FileSystem),
    ("BitOps", StdModules::BitOps),
];
pub struct Scope {
    pub global: GlobalScope,
    pub local: Option<LocalScope>,
    pub completer: Option<Completer>,
    user_module_imported: HashSet<String>,
    std_module_imported: [bool; STD_MODULE_COUNT],
    std_module_map: Rc<HashMap<&'static str, StdModules>>,
}
impl Scope {
    pub fn init() -> Self {
        Self {
            global: GlobalScope::init(),
            local: None,
            completer: None,

            user_module_imported: HashSet::<String>::new(),
            std_module_imported: [false; STD_MODULE_COUNT],
            std_module_map: Rc::new(HashMap::from(STD_MODULE_DATA)),
        }
    }
    // inherit self to create new scope
    pub fn new(&self) -> Self {
        Self {
            global: GlobalScope::init(),
            local: None,
            completer: None,

            user_module_imported: HashSet::<String>::new(),
            std_module_imported: [false; STD_MODULE_COUNT],
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

        if !self.std_module_imported[*target_module as usize] {
            self.std_module_imported[*target_module as usize] = true;
            modules::import_resolver(self, target_module, module_name);
        }
        return Ok(());
    }
    // import user defined module
    pub fn import_from_path(&mut self, module_path: &str) -> Result<Value, ()> {
        let mut module_scope = self.new();

        // if module has not been imported
        if self.user_module_imported.get(module_path) == None {
            // execute the module file
            script::run(module_path, &mut module_scope);

            // import modules that imported by module
            for module_name in module_scope.user_module_imported {
                // user-defined modules
                self.import_from_path(&module_name)?;
            }
            for (i, is_imported) in module_scope.std_module_imported.iter().enumerate() {
                // std modules
                if *is_imported {
                    self.import_std(STD_MODULE_DATA[i].0)?;
                }
            }
            // identify this path as imported
            self.user_module_imported.insert(String::from(module_path));
            let module_obj = module_create(module_scope.global);
            Ok(Value::from(module_obj))
        } else {
            Ok(Value::EMPTY)
        }
    }
}

use std::collections::HashMap;
use std::rc::Rc;

use crate::exec::script::run::run_script;
use crate::public::std::modules::{math, array, basic};
use crate::public::std::std::StdModules;
use crate::public::value::function::BuildInFunction;
use crate::public::value::oop::module::{module_create, get_module_name};
use crate::public::value::value::Overload;

use super::build_in;
use super::super::value::value::Value;

pub struct GlobalScope {
    pub build_in_funcs: HashMap<&'static str, Rc<BuildInFunction>>,
    pub variables: HashMap<String, Value>,
}
impl GlobalScope {
    pub fn init() -> GlobalScope {
        GlobalScope {
            build_in_funcs:
                HashMap::<&'static str, Rc<BuildInFunction>>::new(),
            variables:
                build_in::variables(),
        }
    }
}

pub struct LocalScope {
    pub variables: HashMap<String, Value>,
}
impl LocalScope {
    pub fn init() -> LocalScope {
        LocalScope {
            variables: build_in::variables()
        }
    }
}

// --- --- --- --- --- ---

pub struct Scope {
    pub global: GlobalScope,
    pub local: Option<LocalScope>,
    module: HashMap<String, bool>,
    std_module_map: HashMap<&'static str, StdModules>,
}
const STD_MODULE_DATA: [(&str, StdModules); 4] = [
    ("Basic",  StdModules::Basic),
    ("Math" ,  StdModules::Math),
    ("Array",  StdModules::Array),
    ("FS"   ,  StdModules::FileSystem),
];
impl Scope {
    pub fn init() -> Scope {
        let std_module_map =
            HashMap::from(STD_MODULE_DATA);
        Scope {
            global: GlobalScope::init(),
            local: None,
            module: HashMap::<String, bool>::new(),
            std_module_map,
        }
    }
    pub fn import(&mut self, module_name: &str) -> Result<(), ()> {
        let Some(target_module) = self.std_module_map.get(module_name) else {
            println!("Target module '{}' does not exist.", module_name);
            return Err(())
        };

        if let None = self.module.get(module_name) {
            let func_list = match target_module {
                StdModules::Basic  => basic::function_list(),
                StdModules::Math   => math::function_list(),
                StdModules::Array  => array::function_list(),
                StdModules::FileSystem => todo!(),
            };
            self.module.insert(module_name.to_string(), true);
            self.global.build_in_funcs.extend(func_list)
        }
        Ok(())
    }
    pub fn import_from_path(
        &mut self,
        module_path: &str,
    ) -> Result<(), ()>  {
        let mut module_scope = Scope::init();
        let module_name =
            get_module_name(module_path);

        if let None = self.module.get(module_name) {
            // execute the module file
            run_script(
                module_path.to_string(),
                &mut module_scope
            );

            // import modules that imported by module
            for (module, _) in module_scope.module {
                if let Some(_) = self.std_module_map.get(module.as_str()) {
                    self.import(&module)?;
                } else {
                    self.import_from_path(&module)?;
                }
            }

            // regard the whole module as an Object
            let module_obj =
                module_create(module_scope.global);
            // insert the Object as a variable into
            // the global scope.
            self.global.variables.insert(
                module_name.to_string(),
                Value::create(module_obj)
            );

            self.module.insert(
                module_name.to_string(),
                true
            );
        }
        
        Ok(())
    }
}
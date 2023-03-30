use std::collections::HashMap;
use std::rc::Rc;

use crate::public::std::modules::{math, array, basic};
use crate::public::std::std::StdModules;
use crate::public::value::function::BuildInFunction;

use super::build_in;
use super::super::value::value::Value;

pub struct GlobalScope {
    pub build_in_funcs: HashMap<&'static str, Rc<BuildInFunction>>,
    pub variables: HashMap<String, Rc<Value>>,
}
impl GlobalScope {
    pub fn init() -> GlobalScope {
        type BuildInFuncMap =
            HashMap<&'static str, Rc<BuildInFunction>>;
        GlobalScope {
            build_in_funcs: BuildInFuncMap::new(),
            variables: build_in::variables(),
        }
    }
}

pub struct LocalScope {
    pub variables: HashMap<String, Rc<Value>>,
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
    module: Vec<StdModules>,
    module_map: HashMap<&'static str, StdModules>,
}
impl Scope {
    pub fn init() -> Scope {
        let module_map = HashMap::from([
            ("Basic",  StdModules::Basic),
            ("Math" ,  StdModules::Math),
            ("Array",  StdModules::Array),
            ("FS"   ,  StdModules::FileSystem),
        ]);
        Scope {
            global: GlobalScope::init(),
            local: None,
            module: Vec::<StdModules>::new(),
            module_map,
        }
    }
    pub fn import(&mut self, module_name: &str) -> Result<(), ()> {
        let option_target_module =
            self.module_map.get(module_name);
        let target_module = if option_target_module.is_some() {
            option_target_module.unwrap()
        } else {
            println!("Target module '{}' does not exist.", module_name);
            return Err(())
        };

        if !self.module.contains(&target_module) {
            let func_list = match target_module {
                StdModules::Basic  => basic::function_list(),
                StdModules::Math   => math::function_list(),
                StdModules::String => todo!(),
                StdModules::Array  => array::function_list(),
                StdModules::FileSystem => todo!(),
            };

            self.global.build_in_funcs.extend(func_list)
        }
        Ok(())
    }
}
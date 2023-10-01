use std::rc::Rc;

use self::modules::{
    array::ArrayModule, basic::BasicModule, bit_ops::BitOpsModule, file_system::FileSysModule,
    map::MapModule, math::MathModule, string::StringModule, ClassModule, FunctionModule,
    ObjectModule,
};
use super::value::{
    oop::{class::Class, object::Object},
    Value,
};

pub mod modules;

mod utils;

#[derive(PartialEq, Clone, Copy)]
pub enum StdModules {
    Basic,
    Math,
    Array,
    String,
    Map,
    FileSystem,
    BitOps,
}

impl StdModules {
    pub fn get_fn_list(&self) -> Vec<(String, Value)> {
        match self {
            StdModules::Basic => BasicModule::function_list(),
            StdModules::BitOps => BitOpsModule::function_list(),
            _ => unreachable!(),
        }
    }

    pub fn get_obj_entry(&self) -> Object {
        match self {
            StdModules::Math => MathModule::module_object(),
            StdModules::FileSystem => FileSysModule::module_object(),
            _ => unreachable!(),
        }
    }

    pub fn get_cls_entry(&self) -> Rc<Class> {
        match self {
            StdModules::Array => ArrayModule::module_class(),
            StdModules::String => StringModule::module_class(),
            StdModules::Map => MapModule::module_class(),
            _ => unreachable!(),
        }
    }
}

// --- --- --- --- --- ---

pub struct ModuleClass(Option<Rc<Class>>);
pub const EMPTY_MODULE_CLASS: ModuleClass = ModuleClass(None);

impl ModuleClass {
    pub fn is_some_or_init(&mut self, class_cb: fn() -> Class) {
        if self.0.is_none() {
            let class = class_cb();
            self.0 = Some(class.into());
        }
    }

    pub fn unwrap(&self) -> Rc<Class> {
        let value_ref = self.0.as_ref();
        let Some(wraped) = value_ref else {
            unreachable!()
        };
        return wraped.clone();
    }
}

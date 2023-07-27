use std::rc::Rc;

use self::modules::{
    basic::BasicModule,
    bit_ops::BitOpsModule,
    file_system::FileSysModule,
    math::MathModule,
    array::ArrayModule,
    string::StringModule,
    FunctionModule,
    ObjectModule,
    ClassModule,
};
use super::value::{
    oop::{class::Class, object::Object},
    value::Value,
};

pub mod modules;

mod utils;

#[derive(PartialEq, Clone, Copy)]
pub enum StdModules {
    Basic,
    Math,
    Array,
    String,
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
            _ => unreachable!(),
        }
    }
}

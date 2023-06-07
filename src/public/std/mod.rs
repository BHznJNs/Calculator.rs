use crate::public::std::modules::{array, string};

use self::modules::{math, file_system};

use super::value::oop::{object::Object, class::Class};

pub mod modules;

mod utils;

#[derive(PartialEq)]
pub enum StdModules {
    Basic,
    Math,
    Array,
    String,
    FileSystem,
}

impl StdModules {
    pub fn get_obj_entry(&self) -> Object {
        match self {
            StdModules::Math =>
                math::module_object(),
            StdModules::FileSystem =>
                file_system::module_object(),
            _ => unreachable!()
        }
    }

    pub fn get_cls_entry(&self) -> Class {
        match self {
            StdModules::Array =>
                array::module_class(),
            StdModules::String =>
                string::module_class(),
            _ => unreachable!()
        }
    }
}

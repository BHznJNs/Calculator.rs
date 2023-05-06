use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFnEnum;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::function::{BuildInParam, BuildInFunction, Function, Overload};
use crate::public::value::number::Number;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueTypes, Value};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFnEnum,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match func_body {
        BuildInFnEnum::Split => {},
        BuildInFnEnum::Replace => {},
        _ => {}
    };
    todo!()
}

pub fn module_class() -> Class {
    Class {
        properties: vec![String::from("v")],
        method_storage: DataStoragePattern::Map,
        method_list: None,
        method_map: Some(HashMap::from([
            (String::from("split")   , Function::create(SPLIT)),
            (String::from("replace") , Function::create(REPLACE)),
        ]))
    }
}

pub const SPLIT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "divider"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Split,
};
pub const REPLACE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "from"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Void,
            identi: "to"
        }), None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Replace,
};

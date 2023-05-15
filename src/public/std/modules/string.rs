use std::collections::{HashMap, VecDeque};

use crate::public::run_time::build_in::BuildInFnEnum;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInParam, BuildInFunction, Function, Overload as FunctionOverLoad};
use crate::public::value::oop::class::Class;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueType, Value, Overload as ValueOverload};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFnEnum,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match func_body {
        BuildInFnEnum::Split => {
            let self_value = get_val("self", scope)?;
            let str_value = get_self_prop(self_value, String::from("v"))?;
            let divider_value = get_val("divider", scope)?;

            if let (Value::String(str), Value::String(div)) =
                   (str_value, divider_value) {
                let str_refer = str.borrow();
                let div_refer = div.borrow();
                // splited chars
                let res_split =
                if div_refer.is_empty() {
                    str_refer.split(' ')
                } else {
                    let first_ch =
                        div_refer.chars()
                        .next().unwrap();
                    str_refer.split(first_ch)
                };
                // convert splited to Vec<String>
                let mut res_vec =
                    VecDeque::<Value>::new();
                for c in res_split {
                    let c_value = Value::create(c.to_owned());
                    res_vec.push_back(c_value);
                }
                Value::create(res_vec)
            } else {
                Value::create(ArrayLiteral::new())
            }
        },
        BuildInFnEnum::Replace => todo!(),
        BuildInFnEnum::Repeat => todo!(),
        BuildInFnEnum::Join => todo!(),
        BuildInFnEnum::StartWith => todo!(),
        BuildInFnEnum::EndWith => todo!(),
        _ => todo!()
    };
    Ok(result)
}

pub fn module_class() -> Class {
    Class {
        properties: vec![String::from("v")],
        method_storage: DataStoragePattern::Map,
        method_list: None,
        method_map: Some(HashMap::from([
            (String::from("split")      , Function::create(SPLIT)),
            (String::from("replace")    , Function::create(REPLACE)),
            (String::from("repeat")     , Function::create(REPEAT)),
            (String::from("join")       , Function::create(JOIN)),
            (String::from("start_with") , Function::create(START_WITH)),
            (String::from("end_with")   , Function::create(END_WITH)),
        ]))
    }
}

pub const SPLIT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::String,
            identi: "divider"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Split,
};
pub const REPLACE: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "from"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "to"
        }), None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Replace,
};
pub const REPEAT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "num"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Repeat,
};
pub const JOIN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "divider"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::Join,
};
pub const START_WITH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "str"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::StartWith,
};
pub const END_WITH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Void,
            identi: "str"
        }), None, None,
    ],
    lib: StdModules::String, 
    body: BuildInFnEnum::EndWith,
};
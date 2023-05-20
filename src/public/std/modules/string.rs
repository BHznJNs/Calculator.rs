use std::collections::{HashMap, VecDeque};

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInParam, BuildInFunction, Function, Overload as FunctionOverLoad};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueType, Value, Overload as ValueOverload};

use super::super::utils::get_val::get_val;

// pub fn implement(
//     fn_body: &BuildInFnEnum,
//     scope: &mut Scope,
// ) -> Result<Value, ()> {
//     let result = match fn_body {
//         BuildInFnEnum::Split => {
//             let self_value = get_val("self", scope)?;
//             let str_value = get_self_prop(self_value, "v")?;
//             let divider_value = get_val("divider", scope)?;

//             if let (Value::String(str), Value::String(div)) =
//                    (str_value, divider_value) {
//                 let str_refer = str.borrow();
//                 let div_refer = div.borrow();
//                 // splited chars
//                 let res_split =
//                 if div_refer.is_empty() {
//                     str_refer.split(' ')
//                 } else {
//                     let first_ch =
//                         div_refer.chars()
//                         .next().unwrap();
//                     str_refer.split(first_ch)
//                 };
//                 // convert splited to Vec<String>
//                 let mut res_vec =
//                     VecDeque::<Value>::new();
//                 for c in res_split {
//                     let c_value = Value::create(c.to_owned());
//                     res_vec.push_back(c_value);
//                 }
//                 Value::create(res_vec)
//             } else {
//                 Value::create(ArrayLiteral::new())
//             }
//         },
//         BuildInFnEnum::Replace => todo!(),
//         BuildInFnEnum::Repeat => todo!(),
//         BuildInFnEnum::Join => todo!(),
//         BuildInFnEnum::StartWith => todo!(),
//         BuildInFnEnum::EndWith => todo!(),
//         _ => todo!()
//     };
//     Ok(result)
// }

pub fn module_class() -> Class {
    Class {
        properties: vec![Property {
            identi: String::from("v"),
            type__: ValueType::String,
        }],
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

#[derive(PartialEq)]
pub enum StringFn {
    SPLIT,
    REPLACE,
    REPEAT,
    JOIN,
    STARTWITH,
    ENDWITH,
}

impl StringFn {
    pub fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result =
        match self {
            StringFn::SPLIT => {
                let self_value = get_val("self", scope)?;
                let str_value = get_self_prop(self_value, "v")?;
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
            StringFn::REPLACE => todo!(),
            StringFn::REPEAT => todo!(),
            StringFn::JOIN => todo!(),
            StringFn::STARTWITH => todo!(),
            StringFn::ENDWITH => todo!(),
        };
        Ok(result)
    }
}

// --- --- --- --- --- ---

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
    identi: BuildInFnIdenti::String(StringFn::SPLIT),
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
    identi: BuildInFnIdenti::String(StringFn::REPLACE),
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
    identi: BuildInFnIdenti::String(StringFn::REPEAT),
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
    identi: BuildInFnIdenti::String(StringFn::JOIN),
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
    identi: BuildInFnIdenti::String(StringFn::STARTWITH),
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
    identi: BuildInFnIdenti::String(StringFn::ENDWITH),
};
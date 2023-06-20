use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{Overload, Value, ValueType};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq)]
pub enum MathFn {
    SIN,
    COS,
    TAN,
    ASIN,
    ACOS,
    ATAN,
    SINH,
    COSH,
    TANH,
    RAD,
    DEG,
    LOG10,
    LOG2,
    LOG,
    LN,
    EXP,
    ABS,
    SQRT,
    FLOOR,
    ROUND,
}

pub fn module_object() -> Object {
    let sin = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::SIN),
    };
    let cos = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::COS),
    };
    let tan = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::TAN),
    };

    let asin = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::ASIN),
    };
    let acos = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::ACOS),
    };
    let atan = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::ATAN),
    };

    let sinh = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::SINH),
    };
    let cosh = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::COSH),
    };
    let tanh = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::TANH),
    };

    let rad = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::RAD),
    };
    let deg = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::DEG),
    };

    let log10 = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG10),
    };
    let log2 = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG2),
    };
    let log = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "base"),
            BuildInFnParam(ValueType::Number, "natural"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG),
    };
    let ln = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::LN),
    };
    let exp = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::EXP),
    };

    let abs = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::ABS),
    };
    let sqrt = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::SQRT),
    };
    let floor = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::FLOOR),
    };
    let round = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::ROUND),
    };

    Object {
        prototype: None,
        storage_pattern: DataStoragePattern::Map,
        data_list: None,
        data_map: Some(HashMap::from([
            (String::from("sin"), Value::create(sin).into()),
            (String::from("cos"), Value::create(cos).into()),
            (String::from("tan"), Value::create(tan).into()),
            (String::from("asin"), Value::create(asin).into()),
            (String::from("acos"), Value::create(acos).into()),
            (String::from("atan"), Value::create(atan).into()),
            (String::from("sinh"), Value::create(sinh).into()),
            (String::from("cosh"), Value::create(cosh).into()),
            (String::from("tanh"), Value::create(tanh).into()),
            (String::from("rad"), Value::create(rad).into()),
            (String::from("deg"), Value::create(deg).into()),
            (String::from("log10"), Value::create(log10).into()),
            (String::from("log2"), Value::create(log2).into()),
            (String::from("log"), Value::create(log).into()),
            (String::from("ln"), Value::create(ln).into()),
            (String::from("exp"), Value::create(exp).into()),
            (String::from("abs"), Value::create(abs).into()),
            (String::from("sqrt"), Value::create(sqrt).into()),
            (String::from("floor"), Value::create(floor).into()),
            (String::from("round"), Value::create(round).into()),
        ])),
    }
}

impl BuildInFnCall for MathFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            MathFn::LOG => {
                let base = get_val("base", scope)?;
                let natural = get_val("natural", scope)?;

                let base_f = base.get_f64()?;
                let nature_f = natural.get_f64()?;
                nature_f.log(base_f)
            }
            _ => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;

                match self {
                    MathFn::SIN => f.sin(),
                    MathFn::COS => f.cos(),
                    MathFn::TAN => f.tan(),
                    MathFn::ASIN => f.asin(),
                    MathFn::ACOS => f.acos(),
                    MathFn::ATAN => f.atan(),
                    MathFn::SINH => f.sinh(),
                    MathFn::COSH => f.cosh(),
                    MathFn::TANH => f.tanh(),
                    MathFn::RAD => f.to_radians(),
                    MathFn::DEG => f.to_degrees(),
                    MathFn::LOG10 => f.log10(),
                    MathFn::LOG2 => f.log2(),
                    MathFn::LN => f.ln(),
                    MathFn::EXP => f.exp(),
                    MathFn::ABS => f.abs(),
                    MathFn::SQRT => f.sqrt(),
                    MathFn::FLOOR => f.floor(),
                    MathFn::ROUND => f.round(),
                    _ => unreachable!(),
                }
            }
        };
        Ok(Value::create(result))
    }
}

use std::collections::HashMap;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInFunction, Param};
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
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::SIN),
    };
    let cos = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::COS),
    };
    let tan = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::TAN),
    };

    let asin = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::ASIN),
    };
    let acos = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::ACOS),
    };
    let atan = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::ATAN),
    };

    let sinh = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::SINH),
    };
    let cosh = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::COSH),
    };
    let tanh = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::TANH),
    };

    let rad = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::RAD),
    };
    let deg = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::DEG),
    };

    let log10 = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG10),
    };
    let log2 = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG2),
    };
    let log = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "base",
            },
            Param {
                type__: ValueType::Number,
                identi: "natural",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG),
    };
    let ln = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::LN),
    };
    let exp = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::EXP),
    };

    let abs = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::ABS),
    };
    let sqrt = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::SQRT),
    };
    let floor = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::FLOOR),
    };
    let round = BuildInFunction {
        params: vec![
            Param {
                type__: ValueType::Object,
                identi: "self",
            },
            Param {
                type__: ValueType::Number,
                identi: "input",
            },
        ],
        identi: BuildInFnIdenti::Math(MathFn::ROUND),
    };

    Object {
        prototype: None,
        storage_pattern: DataStoragePattern::Map,
        data_list: None,
        data_map: Some(HashMap::from([
            (
                String::from("sin"),
                Value::create(sin).into(),
            ),
            (
                String::from("cos"),
                Value::create(cos).into(),
            ),
            (
                String::from("tan"),
                Value::create(tan).into(),
            ),
            (
                String::from("asin"),
                Value::create(asin).into(),
            ),
            (
                String::from("acos"),
                Value::create(acos).into(),
            ),
            (
                String::from("atan"),
                Value::create(atan).into(),
            ),
            (
                String::from("sinh"),
                Value::create(sinh).into(),
            ),
            (
                String::from("cosh"),
                Value::create(cosh).into(),
            ),
            (
                String::from("tanh"),
                Value::create(tanh).into(),
            ),
            (
                String::from("rad"),
                Value::create(rad).into(),
            ),
            (
                String::from("deg"),
                Value::create(deg).into(),
            ),
            (
                String::from("log10"),
                Value::create(log10).into(),
            ),
            (
                String::from("log2"),
                Value::create(log2).into(),
            ),
            (
                String::from("log"),
                Value::create(log).into(),
            ),
            (
                String::from("ln"),
                Value::create(ln).into(),
            ),
            (
                String::from("exp"),
                Value::create(exp).into(),
            ),
            (
                String::from("abs"),
                Value::create(abs).into(),
            ),
            (
                String::from("sqrt"),
                Value::create(sqrt).into(),
            ),
            (
                String::from("floor"),
                Value::create(floor).into(),
            ),
            (
                String::from("round"),
                Value::create(round).into(),
            ),
        ])),
    }
}

impl BuildInFnCall for MathFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = match self {
            MathFn::SIN => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sin()
            }
            MathFn::COS => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.cos()
            }
            MathFn::TAN => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.tan()
            }
            MathFn::ASIN => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.asin()
            }
            MathFn::ACOS => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.acos()
            }
            MathFn::ATAN => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.atan()
            }
            MathFn::SINH => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sinh()
            }
            MathFn::COSH => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.cosh()
            }
            MathFn::TANH => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.tanh()
            }
            MathFn::RAD => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.to_radians()
            }
            MathFn::DEG => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.to_degrees()
            }
            MathFn::LOG10 => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.log10()
            }
            MathFn::LOG2 => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sin()
            }
            MathFn::LOG => {
                let base = get_val("base", scope)?;
                let natural = get_val("natural", scope)?;

                let base_f = base.get_f64()?;
                let nature_f = natural.get_f64()?;
                nature_f.log(base_f)
            }
            MathFn::LN => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.ln()
            }
            MathFn::EXP => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.exp()
            }
            MathFn::ABS => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.abs()
            }
            MathFn::SQRT => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sqrt()
            }
            MathFn::FLOOR => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.floor()
            }
            MathFn::ROUND => {
                let input = get_val("input", scope)?;
                let f = input.get_f64()?;
                f.round()
            }
        };
        Ok(Value::create(result))
    }
}

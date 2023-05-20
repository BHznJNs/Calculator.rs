use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueType, Value, Overload};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

pub fn module_object() -> Object {
    Object {
        prototype: None,
        storage_pattern: DataStoragePattern::Map,
        data_list: None,
        data_map: Some(HashMap::from([
            (String::from("sin") , Rc::new(RefCell::new(Value::create(SIN)))),
            (String::from("cos") , Rc::new(RefCell::new(Value::create(COS)))),
            (String::from("tan") , Rc::new(RefCell::new(Value::create(TAN)))),
            (String::from("asin"), Rc::new(RefCell::new(Value::create(ASIN)))),
            (String::from("acos"), Rc::new(RefCell::new(Value::create(ACOS)))),
            (String::from("atan"), Rc::new(RefCell::new(Value::create(ATAN)))),
            (String::from("sinh"), Rc::new(RefCell::new(Value::create(SINH)))),
            (String::from("cosh"), Rc::new(RefCell::new(Value::create(COSH)))),
            (String::from("tanh"), Rc::new(RefCell::new(Value::create(TANH)))),
        
            (String::from("rad"),  Rc::new(RefCell::new(Value::create(RAD)))),
            (String::from("deg"),  Rc::new(RefCell::new(Value::create(DEG)))),
        
            (String::from("log10"), Rc::new(RefCell::new(Value::create(LOG10)))),
            (String::from("log2") , Rc::new(RefCell::new(Value::create(LOG2)))),
            (String::from("log")  , Rc::new(RefCell::new(Value::create(LOG)))),
            (String::from("ln")   , Rc::new(RefCell::new(Value::create(LN)))),
            (String::from("exp")  , Rc::new(RefCell::new(Value::create(EXP)))),
            (String::from("abs")  , Rc::new(RefCell::new(Value::create(ABS)))),
            (String::from("sqrt") , Rc::new(RefCell::new(Value::create(SQRT)))),
            (String::from("floor"), Rc::new(RefCell::new(Value::create(FLOOR)))),
            (String::from("round"), Rc::new(RefCell::new(Value::create(ROUND)))),
        ])),
    }
}

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

impl BuildInFnCall for MathFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result =
        match self {
            MathFn::SIN => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sin()
            },
            MathFn::COS => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.cos()
            },
            MathFn::TAN => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.tan()
            },
            MathFn::ASIN => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.asin()
            },
            MathFn::ACOS => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.acos()
            },
            MathFn::ATAN => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.atan()
            },
            MathFn::SINH => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sinh()
            },
            MathFn::COSH => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.cosh()
            },
            MathFn::TANH => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.tanh()
            },
            MathFn::RAD => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.to_radians()
            },
            MathFn::DEG => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.to_degrees()
            },
            MathFn::LOG10 => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.log10()
            },
            MathFn::LOG2 => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sin()
            },
            MathFn::LOG => {
                let base =
                    get_val("base", scope)?;
                let natural =
                    get_val("natural", scope)?;
    
                let base_f = base.get_f64()?;
                let nature_f = natural.get_f64()?;
                nature_f.log(base_f)
            },
            MathFn::LN => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.ln()
            },
            MathFn::EXP => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.exp()
            },
            MathFn::ABS => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.abs()
            },
            MathFn::SQRT => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.sqrt()
            },
            MathFn::FLOOR => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.floor()
            },
            MathFn::ROUND => {
                let input =
                    get_val("input", scope)?;
                let f = input.get_f64()?;
                f.round()
            },
        };
        Ok(Value::create(result))
    }
}

// --- --- --- --- --- ---

pub const SIN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::SIN),
};
pub const COS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::COS),
};
pub const TAN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::TAN),
};

pub const ASIN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::ASIN),
};
pub const ACOS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::ACOS),
};
pub const ATAN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::ATAN),
};

pub const SINH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::SINH),
};
pub const COSH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::COSH),
};
pub const TANH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::TANH),
};

pub const RAD: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::RAD),
};
pub const DEG: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::DEG),
};

pub const LOG10: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::LOG10),
};
pub const LOG2: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::LOG2),
};
pub const LOG: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "base"
        }),
        Some(BuildInParam {
            type__: ValueType::Number,
            identi: "natural"
        }),
        None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::LOG),
};
pub const LN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::LN),
};
pub const EXP: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::EXP),
};

pub const ABS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::ABS),
};
pub const SQRT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::SQRT),
};
pub const FLOOR: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::FLOOR),
};
pub const ROUND: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueType::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueType::Number,
            identi: "input"
        }), None, None,
    ],
    identi: BuildInFnIdenti::Math(MathFn::ROUND),
};
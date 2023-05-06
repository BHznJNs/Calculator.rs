use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFnEnum;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueTypes, Value, Overload};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFnEnum,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match func_body {
        BuildInFnEnum::Sin => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFnEnum::Cos => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.cos()
        },
        BuildInFnEnum::Tan => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.tan()
        },
        BuildInFnEnum::Asin => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.asin()
        },
        BuildInFnEnum::Acos => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.acos()
        },
        BuildInFnEnum::Atan => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.atan()
        },
        BuildInFnEnum::Sinh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sinh()
        },
        BuildInFnEnum::Cosh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.cosh()
        },
        BuildInFnEnum::Tanh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.tanh()
        },
        BuildInFnEnum::Rad => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.to_radians()
        },
        BuildInFnEnum::Deg => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.to_degrees()
        },
        BuildInFnEnum::Log10 => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.log10()
        },
        BuildInFnEnum::Log2 => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFnEnum::Log => {
            let base =
                get_val("base", scope)?;
            let natural =
                get_val("natural", scope)?;

            let base_f = base.get_f64()?;
            let nature_f = natural.get_f64()?;
            nature_f.log(base_f)
        },
        BuildInFnEnum::Ln => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.ln()
        },
        BuildInFnEnum::Exp => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.exp()
        },
        BuildInFnEnum::Abs => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.abs()
        },
        BuildInFnEnum::Sqrt => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sqrt()
        },
        BuildInFnEnum::Floor => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.floor()
        },
        BuildInFnEnum::Round => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.round()
        },
        _ => {
            println!("Unexpected function in math implement.");
            return Err(())
        }
    };
    Ok(Value::create(result))
}

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

pub const SIN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math, 
    body: BuildInFnEnum::Sin,
};
pub const COS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Cos,
};
pub const TAN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Tan,
};

pub const ASIN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Asin,
};
pub const ACOS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Acos,
};
pub const ATAN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Atan,
};

pub const SINH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Sinh,
};
pub const COSH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Cosh,
};
pub const TANH: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Tanh,
};

pub const RAD: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Rad,
};
pub const DEG: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Deg,
};

pub const LOG10: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Log10,
};
pub const LOG2: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Log2,
};
pub const LOG: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "base"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "natural"
        }),
        None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Log,
};
pub const LN: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Ln,
};
pub const EXP: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Exp,
};

pub const ABS: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Abs,
};
pub const SQRT: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Sqrt,
};
pub const FLOOR: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Floor,
};
pub const ROUND: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Object,
            identi: "self"
        }), Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "input"
        }), None, None,
    ],
    lib: StdModules::Math,
    body: BuildInFnEnum::Round,
};
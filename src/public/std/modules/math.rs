use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::oop::object::Object;
use crate::public::value::oop::utils::data_storage::DataStoragePattern;
use crate::public::value::value::{ValueTypes, Value, Overload};

use super::super::std::StdModules;
use super::super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    let result = match func_body {
        BuildInFuncs::Sin => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFuncs::Cos => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.cos()
        },
        BuildInFuncs::Tan => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.tan()
        },
        BuildInFuncs::Asin => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.asin()
        },
        BuildInFuncs::Acos => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.acos()
        },
        BuildInFuncs::Atan => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.atan()
        },
        BuildInFuncs::Sinh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sinh()
        },
        BuildInFuncs::Cosh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.cosh()
        },
        BuildInFuncs::Tanh => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.tanh()
        },
        BuildInFuncs::Rad => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.to_radians()
        },
        BuildInFuncs::Deg => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.to_degrees()
        },
        BuildInFuncs::Log10 => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.log10()
        },
        BuildInFuncs::Log2 => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFuncs::Log => {
            let base =
                get_val("base", scope)?;
            let natural =
                get_val("natural", scope)?;

            let base_f = base.get_f64()?;
            let nature_f = natural.get_f64()?;
            nature_f.log(base_f)
        },
        BuildInFuncs::Ln => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.ln()
        },
        BuildInFuncs::Exp => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.exp()
        },
        BuildInFuncs::Abs => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.abs()
        },
        BuildInFuncs::Sqrt => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.sqrt()
        },
        BuildInFuncs::Floor => {
            let input =
                get_val("input", scope)?;
            let f = input.get_f64()?;
            f.floor()
        },
        BuildInFuncs::Round => {
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
    body: BuildInFuncs::Sin,
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
    body: BuildInFuncs::Cos,
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
    body: BuildInFuncs::Tan,
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
    body: BuildInFuncs::Asin,
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
    body: BuildInFuncs::Acos,
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
    body: BuildInFuncs::Atan,
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
    body: BuildInFuncs::Sinh,
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
    body: BuildInFuncs::Cosh,
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
    body: BuildInFuncs::Tanh,
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
    body: BuildInFuncs::Rad,
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
    body: BuildInFuncs::Deg,
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
    body: BuildInFuncs::Log10,
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
    body: BuildInFuncs::Log2,
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
    body: BuildInFuncs::Log,
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
    body: BuildInFuncs::Ln,
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
    body: BuildInFuncs::Exp,
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
    body: BuildInFuncs::Abs,
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
    body: BuildInFuncs::Sqrt,
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
    body: BuildInFuncs::Floor,
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
    body: BuildInFuncs::Round,
};
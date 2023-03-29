use std::rc::Rc;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::value::{ValueTypes, Value, Overload};

use super::std::StdModules;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    fn get_input(
        scope: &mut Scope
    ) -> Result<Rc<Value>, ()> {
        let val =
            scope.local
            .as_ref().unwrap()
            .variables
            .get("input");
        match val {
            Some(rc_val) =>
                Ok(rc_val.clone()),
            None => {
                println!("Input for function is missing.");
                Err(())
            },
        }
    }

    let result = match func_body {
        BuildInFuncs::Sin => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFuncs::Cos => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.cos()
        },
        BuildInFuncs::Tan => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.tan()
        },
        BuildInFuncs::Asin => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.asin()
        },
        BuildInFuncs::Acos => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.acos()
        },
        BuildInFuncs::Atan => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.atan()
        },
        BuildInFuncs::Sinh => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.sinh()
        },
        BuildInFuncs::Cosh => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.cosh()
        },
        BuildInFuncs::Tanh => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.tanh()
        },
        BuildInFuncs::Rad => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.to_radians()
        },
        BuildInFuncs::Deg => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.to_degrees()
        },
        BuildInFuncs::Log10 => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.log10()
        },
        BuildInFuncs::Log2 => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.sin()
        },
        BuildInFuncs::Log => {
            let base = scope
                .local.as_ref().unwrap()
                .variables.get("base").unwrap();
            let nature = scope
                .local.as_ref().unwrap()
                .variables.get("natural").unwrap();

            let base_f = base.get_f64()?;
            let nature_f = nature.get_f64()?;
            nature_f.log(base_f)
        },
        BuildInFuncs::Ln => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.ln()
        },
        BuildInFuncs::Exp => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.exp()
        },
        BuildInFuncs::Abs => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.abs()
        },
        BuildInFuncs::Sqrt => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.sqrt()
        },
        BuildInFuncs::Floor => {
            let input =
                get_input(scope)?;
            let f = input.get_f64()?;
            f.floor()
        },
        BuildInFuncs::Round => {
            let input =
                get_input(scope)?;
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

pub fn function_list() -> Vec<(&'static str, Rc<BuildInFunction>)> {
    vec![
        ("sin" , Rc::new(SIN)),
        ("cos" , Rc::new(COS)),
        ("tan" , Rc::new(TAN)),
        ("asin", Rc::new(ASIN)),
        ("acos", Rc::new(ACOS)),
        ("atan", Rc::new(ATAN)),
        ("sinh", Rc::new(SINH)),
        ("cosh", Rc::new(COSH)),
        ("tanh", Rc::new(TANH)),
    
        ("rad",  Rc::new(RAD)),
        ("deg",  Rc::new(DEG)),
    
        ("log10", Rc::new(LOG10)),
        ("log2" , Rc::new(LOG2)),
        ("log"  , Rc::new(LOG)),
        ("ln"   , Rc::new(LN)),
        ("exp"  , Rc::new(EXP)),
        ("abs"  , Rc::new(ABS)),
        ("sqrt" , Rc::new(SQRT)),
        ("floor", Rc::new(FLOOR)),
        ("round", Rc::new(ROUND)),
    ]
}

pub const SIN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math, 
    body: BuildInFuncs::Sin,
};
pub const COS: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Cos,
};
pub const TAN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Tan,
};

pub const ASIN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Asin,
};
pub const ACOS: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Acos,
};
pub const ATAN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Atan,
};

pub const SINH: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Sinh,
};
pub const COSH: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Cosh,
};
pub const TANH: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Tanh,
};

pub const RAD: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Rad,
};
pub const DEG: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Deg,
};

pub const LOG10: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Log10,
};
pub const LOG2: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Log2,
};
pub const LOG: BuildInFunction = BuildInFunction {
    params: [
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "base"
        }),
        Some(BuildInParam {
            type__: ValueTypes::Number,
            identi: "natural"
        }),
        None
    ],
    lib: StdModules::Math,
    body: BuildInFuncs::Log,
};
pub const LN: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Ln,
};
pub const EXP: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Exp,
};

pub const ABS: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Abs,
};
pub const SQRT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Sqrt,
};
pub const FLOOR: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Floor,
};
pub const ROUND: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Math,
    body: BuildInFuncs::Round,
};
use std::cell::Ref;
use std::rc::Rc;
use std::str::FromStr;

use crate::public::run_time::build_in::BuildInFuncs;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInParam, BuildInFunction};
use crate::public::value::value::{ValueTypes, Value, Overload};

use super::std::StdModules;
use super::utils::get_val::get_val;

pub fn implement(
    func_body: &BuildInFuncs,
    scope: &mut Scope,
) -> Result<Value, ()> {
    fn str_to_num<T: FromStr>(
        str: Ref<String>
    ) -> Result<T, ()> {
        // i64 || f64
        match str.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => {
                println!("Invalid string coverting to number.");
                return Err(())
            },
        }
    }

    let result = match func_body {
        BuildInFuncs::Int => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input.as_ref() {
                let refer = str.borrow();
                let i = str_to_num::<i64>(refer)?;
                Value::create(i)
            } else {
                println!("Invalid param type: expected String.");
                return Err(())
            }
        },
        BuildInFuncs::Float => {
            let input =
                get_val("input", scope)?;

            if let Value::String(str) = input.as_ref() {
                let refer = str.borrow();
                let f = str_to_num::<f64>(refer)?;
                Value::create(f)
            } else {
                println!("Invalid param type: expected String.");
                return Err(())
            }
        },
        BuildInFuncs::String => {
            let input =
                get_val("input", scope)?;

            if let Value::Number(num) = input.as_ref() {
                let str = num.to_string();
                Value::create(str)
            } else {
                println!("Invalid param type: expected Number.");
                return Err(())
            }
        },
        _ => {
            println!("Unexpected function in math implement.");
            return Err(())
        }
    };

    Ok(result)
}

pub fn function_list() -> Vec<(&'static str, Rc<BuildInFunction>)> {
    vec![
        ("int"    , Rc::new(INT)),
        ("float"  , Rc::new(FLOAT)),
        ("string" , Rc::new(STR)),
    ]
}

pub const INT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::String,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Int,
};
pub const FLOAT: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::String,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::Float,
};
pub const STR: BuildInFunction = BuildInFunction {
    params: [Some(BuildInParam {
        type__: ValueTypes::Number,
        identi: "input"
    }), None, None],
    lib: StdModules::Basic, 
    body: BuildInFuncs::String,
};
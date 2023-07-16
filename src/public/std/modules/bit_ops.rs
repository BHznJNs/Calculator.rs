use super::super::utils::get_val::get_val;
use crate::public::run_time::{build_in::BuildInFnIdenti, scope::Scope};
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::value::{Overload, Value, ValueType};

use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
pub enum BitOpsFn {
    AND,
    OR,
    XOR,
    LShift,
    RShift,
    NOT,
}

pub fn function_list() -> Vec<(String, Value)> {
    let and = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Number, "num1"),
            BuildInFnParam(ValueType::Number, "num2"),
        ],
        identi: BuildInFnIdenti::BitOps(BitOpsFn::AND),
    };
    let mut or = and.clone();
    let mut xor = and.clone();
    let mut l_shift = and.clone();
    let mut r_shift = and.clone();
    or.identi = BuildInFnIdenti::BitOps(BitOpsFn::OR);
    xor.identi = BuildInFnIdenti::BitOps(BitOpsFn::XOR);
    l_shift.identi = BuildInFnIdenti::BitOps(BitOpsFn::LShift);
    r_shift.identi = BuildInFnIdenti::BitOps(BitOpsFn::RShift);

    let not = BuildInFunction {
        params: vec![BuildInFnParam(ValueType::Number, "input")],
        identi: BuildInFnIdenti::BitOps(BitOpsFn::NOT),
    };

    return vec![
        (String::from("AND"), Value::create(and)),
        (String::from("OR"), Value::create(or)),
        (String::from("XOR"), Value::create(xor)),
        (String::from("LShift"), Value::create(l_shift)),
        (String::from("RShift"), Value::create(r_shift)),
        (String::from("NOT"), Value::create(not)),
    ];
}

impl BuildInFnCall for BitOpsFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = if *self != BitOpsFn::NOT {
            // AND | OR | XOR | LShift | RShift
            let num_val1 = get_val("num1", scope)?;
            let num_val2 = get_val("num2", scope)?;
            let (compute_num1, compute_num2) = (num_val1.get_i64()?, num_val2.get_i64()?);

            let compute_res = match self {
                BitOpsFn::AND => compute_num1 & compute_num2,
                BitOpsFn::OR => compute_num1 | compute_num2,
                BitOpsFn::XOR => compute_num1 ^ compute_num2,
                BitOpsFn::LShift => compute_num1 << compute_num2,
                BitOpsFn::RShift => compute_num1 >> compute_num2,
                _ => unreachable!(),
            };
            Value::create(compute_res)
        } else {
            // NOT
            let input = get_val("input", scope)?;
            let compute_num = input.get_i64()?;
            Value::create(!compute_num)
        };
        return Ok(result);
    }
}

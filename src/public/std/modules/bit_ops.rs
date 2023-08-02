use super::super::utils::get_val::get_val;
use crate::public::run_time::{build_in::BuildInFnIdenti, scope::Scope};
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::value::{Value, ValueType};

use super::{BuildInFnCall, FunctionModule};

#[derive(PartialEq, Clone)]
pub enum BitOpsModule {
    AND,
    OR,
    XOR,
    LShift,
    RShift,
    NOT,
}

impl FunctionModule for BitOpsModule {
    fn function_list() -> Vec<(String, Value)> {
        let and = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Number, "num1"),
                BuildInFnParam(ValueType::Number, "num2"),
            ],
            identi: BuildInFnIdenti::BitOps(Self::AND),
        };
        let mut or = and.clone();
        let mut xor = and.clone();
        let mut l_shift = and.clone();
        let mut r_shift = and.clone();
        or.identi = BuildInFnIdenti::BitOps(Self::OR);
        xor.identi = BuildInFnIdenti::BitOps(Self::XOR);
        l_shift.identi = BuildInFnIdenti::BitOps(Self::LShift);
        r_shift.identi = BuildInFnIdenti::BitOps(Self::RShift);

        let not = BuildInFunction {
            params: vec![BuildInFnParam(ValueType::Number, "input")],
            identi: BuildInFnIdenti::BitOps(Self::NOT),
        };

        return vec![
            (String::from("AND"), Value::from(and)),
            (String::from("OR"), Value::from(or)),
            (String::from("XOR"), Value::from(xor)),
            (String::from("LShift"), Value::from(l_shift)),
            (String::from("RShift"), Value::from(r_shift)),
            (String::from("NOT"), Value::from(not)),
        ];
    }
}

impl BuildInFnCall for BitOpsModule {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let result = if *self != Self::NOT {
            // AND | OR | XOR | LShift | RShift
            let num_val1 = get_val("num1", scope)?;
            let num_val2 = get_val("num2", scope)?;
            let (compute_num1, compute_num2) = (num_val1.get_i64()?, num_val2.get_i64()?);

            let compute_result = match self {
                Self::AND => compute_num1 & compute_num2,
                Self::OR => compute_num1 | compute_num2,
                Self::XOR => compute_num1 ^ compute_num2,
                Self::LShift => compute_num1 << compute_num2,
                Self::RShift => compute_num1 >> compute_num2,
                _ => unreachable!(),
            };
            Value::from(compute_result)
        } else {
            // NOT
            let input = get_val("input", scope)?;
            let compute_num = input.get_i64()?;
            Value::from(!compute_num)
        };
        return Ok(result);
    }
}

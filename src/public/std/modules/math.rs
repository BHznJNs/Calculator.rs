use std::rc::Rc;

use crate::public::error::{internal_error, InternalComponent};
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{BuildInFnParam, BuildInFunction, Function};
use crate::public::value::number::Number;
use crate::public::value::oop::class::Class;
use crate::public::value::oop::object::Object;
use crate::public::value::value::{Value, ValueType};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
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
    MOD,
    Fraction,
}

static mut MODULE_CLASS: Option<Rc<Class>> = None;

fn static_class_setter() {
    let sin = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "input"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::SIN),
    };
    let mut cos = sin.clone();
    let mut tan = sin.clone();
    let mut asin = sin.clone();
    let mut acos = sin.clone();
    let mut atan = sin.clone();
    let mut sinh = sin.clone();
    let mut cosh = sin.clone();
    let mut tanh = sin.clone();
    let mut rad = sin.clone();
    let mut deg = sin.clone();
    let mut log10 = sin.clone();
    let mut log2 = sin.clone();
    let mut ln = sin.clone();
    let mut exp = sin.clone();
    let mut abs = sin.clone();
    let mut sqrt = sin.clone();
    let mut floor = sin.clone();
    let mut round = sin.clone();

    cos.identi = BuildInFnIdenti::Math(MathFn::COS);
    tan.identi = BuildInFnIdenti::Math(MathFn::TAN);
    asin.identi = BuildInFnIdenti::Math(MathFn::ASIN);
    acos.identi = BuildInFnIdenti::Math(MathFn::ACOS);
    atan.identi = BuildInFnIdenti::Math(MathFn::ATAN);
    sinh.identi = BuildInFnIdenti::Math(MathFn::SINH);
    cosh.identi = BuildInFnIdenti::Math(MathFn::COSH);
    tanh.identi = BuildInFnIdenti::Math(MathFn::TANH);
    rad.identi = BuildInFnIdenti::Math(MathFn::RAD);
    deg.identi = BuildInFnIdenti::Math(MathFn::DEG);
    log10.identi = BuildInFnIdenti::Math(MathFn::LOG10);
    log2.identi = BuildInFnIdenti::Math(MathFn::LOG2);
    ln.identi = BuildInFnIdenti::Math(MathFn::LN);
    exp.identi = BuildInFnIdenti::Math(MathFn::EXP);
    abs.identi = BuildInFnIdenti::Math(MathFn::ABS);
    sqrt.identi = BuildInFnIdenti::Math(MathFn::SQRT);
    floor.identi = BuildInFnIdenti::Math(MathFn::FLOOR);
    round.identi = BuildInFnIdenti::Math(MathFn::ROUND);

    let log = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "base"),
            BuildInFnParam(ValueType::Number, "natural"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::LOG),
    };
    let modulo = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "base"),
            BuildInFnParam(ValueType::Number, "target"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::MOD),
    };
    let fraction = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Number, "upper"),
            BuildInFnParam(ValueType::Number, "lower"),
        ],
        identi: BuildInFnIdenti::Math(MathFn::Fraction),
    };

    unsafe {
        MODULE_CLASS = Some(
            Class::new(
                vec![],
                vec![
                    (String::from("sin"), Function::from(sin)),
                    (String::from("cos"), Function::from(cos)),
                    (String::from("tan"), Function::from(tan)),
                    (String::from("asin"), Function::from(asin)),
                    (String::from("acos"), Function::from(acos)),
                    (String::from("atan"), Function::from(atan)),
                    (String::from("sinh"), Function::from(sinh)),
                    (String::from("cosh"), Function::from(cosh)),
                    (String::from("tanh"), Function::from(tanh)),
                    (String::from("rad"), Function::from(rad)),
                    (String::from("deg"), Function::from(deg)),
                    (String::from("log10"), Function::from(log10)),
                    (String::from("log2"), Function::from(log2)),
                    (String::from("log"), Function::from(log)),
                    (String::from("ln"), Function::from(ln)),
                    (String::from("exp"), Function::from(exp)),
                    (String::from("abs"), Function::from(abs)),
                    (String::from("sqrt"), Function::from(sqrt)),
                    (String::from("floor"), Function::from(floor)),
                    (String::from("round"), Function::from(round)),
                    (String::from("mod"), Function::from(modulo)),
                    (String::from("fraction"), Function::from(fraction)),
                ],
            )
            .into(),
        )
    }
}

pub fn module_object() -> Object {
    if unsafe { MODULE_CLASS == None } {
        static_class_setter();
    }

    return Class::instantiate(
        unsafe { MODULE_CLASS.as_ref().unwrap().clone() },
        ArrayLiteral::new(),
    )
    .unwrap();
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
            MathFn::MOD => {
                let base = get_val("base", scope)?;
                let target = get_val("target", scope)?;

                let (Value::Number(number1), Value::Number(number2)) = (base, target) else {
                    unreachable!()
                };
                // the latter number can not be ZERO
                if number2.float_value() == 0.0 {
                    return Ok(Value::Number(Number::NotANumber));
                }
                if let (Number::Int(i1), Number::Int(i2)) = (number1, number2) {
                    return Ok(Value::Number(Number::Int(i1 % i2)));
                } else {
                    let f1 = number1.float_value();
                    let f2 = number2.float_value();
                    f1 % f2
                }
            }
            MathFn::Fraction => {
                let upper_value = get_val("upper", scope)?;
                let lower_value = get_val("lower", scope)?;

                if let (Value::Number(Number::Int(upper)), Value::Number(Number::Int(lower))) = (upper_value, lower_value) {
                    return Ok(Value::Number(Number::Fraction(upper, lower)));
                } else {
                    return Err(internal_error(
                        InternalComponent::Std,
                        "two Int typed value is expected"
                    )?)
                }
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
        Ok(Value::from(result))
    }
}

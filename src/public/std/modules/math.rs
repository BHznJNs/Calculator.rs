use crate::public::error::{math_error, CalcResult};
use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::value::function::{BuildInFnParam, BuildInFunction};
use crate::public::value::number::{complex, Number};
use crate::public::value::oop::object::Object;
use crate::public::value::{Value, ValueType};

use super::super::utils::get_val::get_val;
use super::{BuildInFnCall, ObjectModule};

#[derive(PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum MathModule {
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
    REAL,
    IMAGINARY,
}

impl ObjectModule for MathModule {
    fn module_object() -> Object {
        let sin = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "input"),
            ],
            identi: BuildInFnIdenti::Math(Self::SIN),
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
        let mut real = sin.clone();
        let mut imaginary = sin.clone();

        cos.identi = BuildInFnIdenti::Math(Self::COS);
        tan.identi = BuildInFnIdenti::Math(Self::TAN);
        asin.identi = BuildInFnIdenti::Math(Self::ASIN);
        acos.identi = BuildInFnIdenti::Math(Self::ACOS);
        atan.identi = BuildInFnIdenti::Math(Self::ATAN);
        sinh.identi = BuildInFnIdenti::Math(Self::SINH);
        cosh.identi = BuildInFnIdenti::Math(Self::COSH);
        tanh.identi = BuildInFnIdenti::Math(Self::TANH);
        rad.identi = BuildInFnIdenti::Math(Self::RAD);
        deg.identi = BuildInFnIdenti::Math(Self::DEG);
        log10.identi = BuildInFnIdenti::Math(Self::LOG10);
        log2.identi = BuildInFnIdenti::Math(Self::LOG2);
        ln.identi = BuildInFnIdenti::Math(Self::LN);
        exp.identi = BuildInFnIdenti::Math(Self::EXP);
        abs.identi = BuildInFnIdenti::Math(Self::ABS);
        sqrt.identi = BuildInFnIdenti::Math(Self::SQRT);
        floor.identi = BuildInFnIdenti::Math(Self::FLOOR);
        round.identi = BuildInFnIdenti::Math(Self::ROUND);
        real.identi = BuildInFnIdenti::Math(Self::REAL);
        imaginary.identi = BuildInFnIdenti::Math(Self::IMAGINARY);

        let log = BuildInFunction {
            params: vec![
                BuildInFnParam(ValueType::Object, "self"),
                BuildInFnParam(ValueType::Number, "base"),
                BuildInFnParam(ValueType::Number, "natural"),
            ],
            identi: BuildInFnIdenti::Math(Self::LOG),
        };

        let module_obj_props = vec![
            (String::from("sin"), Value::from(sin)),
            (String::from("cos"), Value::from(cos)),
            (String::from("tan"), Value::from(tan)),
            (String::from("asin"), Value::from(asin)),
            (String::from("acos"), Value::from(acos)),
            (String::from("atan"), Value::from(atan)),
            (String::from("sinh"), Value::from(sinh)),
            (String::from("cosh"), Value::from(cosh)),
            (String::from("tanh"), Value::from(tanh)),
            (String::from("rad"), Value::from(rad)),
            (String::from("deg"), Value::from(deg)),
            (String::from("log10"), Value::from(log10)),
            (String::from("log2"), Value::from(log2)),
            (String::from("log"), Value::from(log)),
            (String::from("ln"), Value::from(ln)),
            (String::from("exp"), Value::from(exp)),
            (String::from("abs"), Value::from(abs)),
            (String::from("sqrt"), Value::from(sqrt)),
            (String::from("floor"), Value::from(floor)),
            (String::from("round"), Value::from(round)),
            (String::from("real"), Value::from(real)),
            (String::from("imag"), Value::from(imaginary)),
        ];
        return Object::new(module_obj_props, None);
    }
}

impl BuildInFnCall for MathModule {
    fn call(&self, scope: &mut Scope) -> CalcResult<Value> {
        fn log_resolver(scope: &mut Scope) -> CalcResult<Value> {
            let is_valid_base = |base: f64| base > 0.0 || base != 1.0;
            let is_valid_natural = |natural: f64| natural > 0.0;
            let Value::Number(base) = get_val("base", scope)? else {
                unreachable!()
            };
            let Value::Number(natural) = get_val("natural", scope)? else {
                unreachable!()
            };

            if !base.is_real() || !natural.is_real() {
                // for complex logarithm
                let ((real_b, imaginary_b), (real_n, imaginary_n)) =
                    complex::pre_process(base, natural);
                let (real, imaginary) = complex::log(real_b, imaginary_b, real_n, imaginary_n);
                return Ok(Value::Number(Number::Complex(real, imaginary)));
            }

            let base_f = base.float_value();
            let nature_f = natural.float_value();

            if !is_valid_base(base_f) || !is_valid_natural(nature_f) {
                return Err(math_error("invalid logarithm operation"));
            }
            let result = nature_f.log(base_f);
            return Ok(Value::from(result));
        }

        fn complex_math_resolver(
            math_op: &MathModule,
            real: f64,
            imaginary: f64,
        ) -> CalcResult<Number> {
            let result = match math_op {
                MathModule::SIN => Number::complex(complex::sine(real, imaginary)),
                MathModule::COS => Number::complex(complex::cosine(real, imaginary)),
                MathModule::TAN => {
                    Number::complex(complex::sine(real, imaginary))
                        / Number::complex(complex::cosine(real, imaginary))
                }
                MathModule::ASIN => Number::complex(complex::arcsine(real, imaginary)),
                MathModule::ACOS => Number::complex(complex::arccosine(real, imaginary)),
                MathModule::ATAN => Number::complex(complex::arctangent(real, imaginary)),
                MathModule::SINH => Number::complex(complex::sine_hyperbolic(real, imaginary)),
                MathModule::COSH => Number::complex(complex::cosine_hyperbolic(real, imaginary)),
                MathModule::TANH => {
                    Number::complex(complex::sine_hyperbolic(real, imaginary))
                        / Number::complex(complex::cosine_hyperbolic(real, imaginary))
                }

                MathModule::RAD => {
                    return Err(math_error(
                        "math function \"rag\" is invalid for complex input",
                    ))
                }
                MathModule::DEG => {
                    return Err(math_error(
                        "math function \"deg\" is invalid for complex input",
                    ))
                }
                MathModule::LOG10 => Number::complex(complex::pow(10.0, 0.0, real, imaginary)),
                MathModule::LOG2 => Number::complex(complex::pow(2.0, 0.0, real, imaginary)),
                MathModule::LN => Number::complex(complex::ln(real, imaginary)),
                MathModule::EXP => Number::complex(complex::exp(real, imaginary)),
                MathModule::SQRT => Number::complex(complex::sqrt(real, imaginary)),
                MathModule::ABS => Number::Complex(real.abs(), imaginary.abs()),
                MathModule::FLOOR => Number::Complex(real.floor(), imaginary.floor()),
                MathModule::ROUND => Number::Complex(real.round(), imaginary.round()),
                _ => unreachable!(),
            };
            return Ok(result);
        }

        // the `log` is a special case with two params
        if matches!(self, Self::LOG) {
            return log_resolver(scope);
        }

        let input = get_val("input", scope)?;
        let Value::Number(number) = input else {
            unreachable!()
        };

        match self {
            Self::REAL => return Ok(Value::from(number.real())),
            Self::IMAGINARY => return Ok(Value::from(number.imaginary())),
            _ => {}
        }

        if let Number::Complex(real, imaginary) = number {
            let calc_result = complex_math_resolver(self, real, imaginary)?;
            return Ok(Value::Number(calc_result));
        }

        // for non-complex input calculation
        let f = number.float_value();
        let result = match self {
            Self::SIN => f.sin(),
            Self::COS => f.cos(),
            Self::TAN => f.tan(),
            Self::SINH => f.sinh(),
            Self::COSH => f.cosh(),
            Self::TANH => f.tanh(),
            Self::RAD => f.to_radians(),
            Self::DEG => f.to_degrees(),
            Self::LOG10 => f.log10(),
            Self::LOG2 => f.log2(),
            Self::LN => f.ln(),
            Self::EXP => f.exp(),
            Self::ABS => f.abs(),
            Self::SQRT => f.sqrt(),
            Self::FLOOR => f.floor(),
            Self::ROUND => f.round(),

            Self::ASIN | Self::ACOS | Self::ATAN => {
                // `f` less than -1 and greater than 1
                if !(-1.0..=1.0).contains(&f) {
                    // inverse trigonometric function error.
                    return Err(
                        math_error(
                            "the input for inverse trigonometric function should be less than 1 and greater than -1"
                        )
                    );
                }
                match self {
                    Self::ASIN => f.asin(),
                    Self::ACOS => f.acos(),
                    Self::ATAN => f.atan(),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        return Ok(Value::from(result));
    }
}

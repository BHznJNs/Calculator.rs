use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::consts::E;

use crate::public::value::number::Number;

pub struct BuildIn {
    pub  sin: fn(f64) -> f64,
    pub  cos: fn(f64) -> f64,
    pub  tan: fn(f64) -> f64,
    pub asin: fn(f64) -> f64,
    pub acos: fn(f64) -> f64,
    pub atan: fn(f64) -> f64,
    pub sinh: fn(f64) -> f64,
    pub cosh: fn(f64) -> f64,
    pub tanh: fn(f64) -> f64,

    pub rad: fn(f64) -> f64,
    pub deg: fn(f64) -> f64,

    pub log10: fn(f64) -> f64,
    pub log2 : fn(f64) -> f64,
    pub ln   : fn(f64) -> f64,
    pub exp  : fn(f64) -> f64,

    pub abs  : fn(f64) -> f64,
    pub sqrt : fn(f64) -> f64,
    pub floor: fn(f64) -> f64,
    pub round: fn(f64) -> f64,

    pub pi: f64,
    pub e : f64,
}
impl BuildIn {
    pub fn init() -> Self {
        let instance = BuildIn {
             sin: |params: f64| params. sin(),
             cos: |params: f64| params. cos(),
             tan: |params: f64| params. tan(),
            asin: |params: f64| params.asin(),
            acos: |params: f64| params.acos(),
            atan: |params: f64| params.atan(),
            sinh: |params: f64| params.sinh(),
            cosh: |params: f64| params.cosh(),
            tanh: |params: f64| params.tanh(),

            rad:  |params: f64| params.to_radians(),
            deg:  |params: f64| params.to_degrees(),

            log10: |params: f64| params.log10(),
            log2 : |params: f64| params.log2(),
            ln   : |params: f64| params.ln(),
            exp  : |params: f64| params.exp(),

            abs  : |params: f64| params.abs(),
            sqrt : |params: f64| params.sqrt(),
            floor: |params: f64| params.floor(),
            round: |params: f64| params.round(),

            pi: PI,
            e : E,
        };
        return instance
    }
}

pub fn build_in_funcs(build_in_inst: &BuildIn) -> HashMap<&'static str, fn(f64) -> f64> {
    let map = HashMap::from([
        ("sin" , build_in_inst.sin),
        ("cos" , build_in_inst.cos),
        ("tan" , build_in_inst.tan),
        ("asin", build_in_inst.asin),
        ("acos", build_in_inst.acos),
        ("atan", build_in_inst.atan),
        ("sinh", build_in_inst.sinh),
        ("cosh", build_in_inst.cosh),
        ("tanh", build_in_inst.tanh),

        ("rad", build_in_inst.rad),
        ("deg", build_in_inst.deg),

        ("log10", build_in_inst.log10),
        ("log2" , build_in_inst.log2),
        ("ln"   , build_in_inst.ln),
        ("exp"  , build_in_inst.exp),

        ("abs"  , build_in_inst.abs),
        ("sqrt" , build_in_inst.sqrt),
        ("floor", build_in_inst.floor),
        ("round", build_in_inst.round),
    ]);
    return map
}

pub fn variables(build_in_inst: &BuildIn) -> HashMap<String, Number> {
    let map = HashMap::from([
        (String::from("PI"), Number::Float(build_in_inst.pi)),
        (String::from("E") , Number::Float(build_in_inst.e )),
    ]);
    return map
}
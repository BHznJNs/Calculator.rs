use std::f64::consts::PI;
use std::f64::consts::E;

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

    pub log10: fn(f64) -> f64,
    pub log2 : fn(f64) -> f64,
    pub ln   : fn(f64) -> f64,
    pub exp  : fn(f64) -> f64,

    pub abs  : fn(f64) -> f64,

    pub pi: f64,
    pub e : f64,
}
impl BuildIn {
    pub fn init() -> Self {
        let instance = BuildIn {
             sin: |num: f64| num. sin(),
             cos: |num: f64| num. cos(),
             tan: |num: f64| num. tan(),
            asin: |num: f64| num.asin(),
            acos: |num: f64| num.acos(),
            atan: |num: f64| num.atan(),
            sinh: |num: f64| num.sinh(),
            cosh: |num: f64| num.cosh(),
            tanh: |num: f64| num.tanh(),

            log10: |num: f64| num.log10(),
            log2 : |num: f64| num.log2(),
            ln   : |num: f64| num.ln(),
            exp  : |num: f64| num.exp(),

            abs  : |num: f64| num.abs(),

            pi: PI,
            e : E,
        };
        return instance
    }
}
pub struct BuildIn {
    // map: HashMap<String, fn(f64) -> f64>,
    pub  sin: fn(f64) -> f64,
    pub  cos: fn(f64) -> f64,
    pub  tan: fn(f64) -> f64,
    pub asin: fn(f64) -> f64,
    pub acos: fn(f64) -> f64,
    pub atan: fn(f64) -> f64,
    pub sinh: fn(f64) -> f64,
    pub cosh: fn(f64) -> f64,
    pub tanh: fn(f64) -> f64,
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
        };
        return instance
    }
}
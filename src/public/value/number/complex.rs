use super::Number;

pub(super) struct Polar {
    pub magnitude: f64,
    pub phase: f64,
}

impl Polar {
    pub(self) fn new(real: f64, imaginary: f64) -> Self {
        Self {
            magnitude: Self::modulus(real, imaginary),
            phase: Self::phase_angle(real, imaginary),
        }
    }

    pub(self) fn to_cartesian(&self) -> (f64, f64) {
        let real = self.magnitude * self.phase.cos();
        let imaginary = self.magnitude * self.phase.sin();
        return (real, imaginary);
    }

    pub fn modulus(real: f64, imaginary: f64) -> f64 {
        (real.powi(2) + imaginary.powi(2)).sqrt()
    }
    
    pub fn phase_angle(real: f64, imaginary: f64) -> f64 {
        imaginary.atan2(real)
    }
}

pub fn pre_process(base: Number, power: Number) -> ((f64, f64), (f64, f64)) {
    let real_b;
    let imaginary_b;
    let real_p;
    let imaginary_p;

    if let Number::Complex(real, imaginary) = base {
        real_b = real;
        imaginary_b = imaginary;
    } else {
        real_b = base.float_value();
        imaginary_b = 0.0;
    }
    if let Number::Complex(real, imaginary) = power {
        real_p = real;
        imaginary_p = imaginary;
    } else {
        real_p = base.float_value();
        imaginary_p = 0.0;
    }
    return ((real_b, imaginary_b), (real_p, imaginary_p));
}

pub fn pow(real_b: f64, imaginary_b: f64, real_p: f64, imaginary_p: f64) -> Number {
    // to Euler form:
    // real + imaginary * i => magnitude * e ^ (phase * i)
    let mut polar_form = Polar::new(real_b, imaginary_b);

    // (r (cosθ + isinθ)) ^ (c + di) = r ^ c (cos(cθ +d) + isin(cθ + d))
    polar_form.magnitude = polar_form.magnitude.powf(real_p);
    polar_form.phase = real_p * polar_form.phase + imaginary_p;
    let (real, imaginary) = polar_form.to_cartesian();

    return Number::Complex(real, imaginary);
}

pub fn multiply(real_b: f64, imaginary_b: f64, real_p: f64, imaginary_p: f64) -> Number {
    let real = real_b * real_p  - imaginary_b * imaginary_p;
    let imaginary = real_b * imaginary_p + real_p * imaginary_b;
    return Number::Complex(real, imaginary);
}

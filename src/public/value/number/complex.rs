use super::Number;

pub type Complex = (f64, f64);
pub const I: Number = Number::Complex(0.0, 1.0);

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

    pub(self) fn to_cartesian(&self) -> Complex {
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

pub fn pre_process(a: Number, b: Number) -> (Complex, Complex) {
    let real_b;
    let imaginary_b;
    let real_p;
    let imaginary_p;

    if let Number::Complex(real, imaginary) = a {
        real_b = real;
        imaginary_b = imaginary;
    } else {
        real_b = a.float_value();
        imaginary_b = 0.0;
    }
    if let Number::Complex(real, imaginary) = b {
        real_p = real;
        imaginary_p = imaginary;
    } else {
        real_p = b.float_value();
        imaginary_p = 0.0;
    }
    return ((real_b, imaginary_b), (real_p, imaginary_p));
}

pub fn multiply(real_b: f64, imaginary_b: f64, real_p: f64, imaginary_p: f64) -> Complex {
    let real = real_b * real_p  - imaginary_b * imaginary_p;
    let imaginary = real_b * imaginary_p + real_p * imaginary_b;
    return (real, imaginary);
}

// logarithm calculation functions
pub fn ln(real: f64, imaginary: f64) -> Complex {
    // ln(magnitude) + i * phase
    let polar_form = Polar::new(real, imaginary);
    let real = polar_form.magnitude.ln();
    let imaginary = polar_form.phase;
    return (real, imaginary);
}

pub fn log(real_b: f64, imaginary_b: f64, real_n: f64, imaginary_n: f64) -> Complex {
    // log_{a + bi}{c + di} => ln(c + di) / ln(a + bi)
    let upper = Number::complex(ln(real_n, imaginary_n));
    let lower = Number::complex(ln(real_b, imaginary_b));
    let Number::Complex(real, imaginary) = upper / lower else {
        unreachable!()
    };
    return (real, imaginary);
}

// power calculating functions
pub fn pow(real_b: f64, imaginary_b: f64, real_p: f64, imaginary_p: f64) -> Complex {
    // to Euler form:
    // real + imaginary * i => magnitude * e ^ (phase * i)
    let mut polar_form = Polar::new(real_b, imaginary_b);

    // (r (cosθ + isinθ)) ^ (c + di) = r ^ c (cos(cθ +d) + isin(cθ + d))
    polar_form.magnitude = polar_form.magnitude.powf(real_p);
    polar_form.phase = real_p * polar_form.phase + imaginary_p;
    let (real, imaginary) = polar_form.to_cartesian();

    return (real, imaginary);
}

pub fn sqrt(real: f64, imaginary: f64) -> Complex {
    // real      part: s = sqrt((x + hypot(x, y))/2)
    // imaginary part: d = (y/2)/s
    let s = ((real + real.hypot(imaginary)) * 0.5).sqrt();
    let d = imaginary * 0.5 / s;
    return (s, d);
}

pub fn exp(real: f64, imaginary: f64) -> Complex {
    // e ^ x (cos(y) + isin(y))
    let factor = real.exp();
    let real = factor * imaginary.cos();
    let imaginary = factor * imaginary.sin();
    return (real, imaginary);
}

// trigonometric functions
pub fn sine(real: f64, imaginary: f64) -> Complex {
    // sin(x)cosh(y) + icos(x)sinh(y)
    let result_real = real.sin() * imaginary.cosh();
    let result_imaginary = real.cos() * imaginary.sinh();
    return (result_real, result_imaginary);
}
pub fn cosine(real: f64, imaginary: f64) -> Complex {
    // cos(x)cosh(y) - sin(x)sinh(y)
    let result_real = real.cos() * imaginary.cosh();
    let result_imaginary = -1.0 * real.sin() * imaginary.sinh();
    return (result_real, result_imaginary);
}

// inverse trigonometric functions
pub fn arctangent(real: f64, imaginary: f64) -> Complex {
    // (1 / 2i) ln [(i - z) / (i + z)]
    let factor = Number::Float(0.5) / I;
    let z = Number::Complex(real, imaginary);
    let Number::Complex(real, imaginary) = (I - z) / (I + z) else {
        unreachable!()
    };
    let Number::Complex(real, imaginary) = factor * Number::complex(ln(real, imaginary)) else {
        unreachable!()
    };
    return (real, imaginary);
}
pub fn arcsine(real: f64, imaginary: f64) -> Complex {
    // (1 / i) ln(iz + (1 - z ^ 2) ^ (1 / 2))
    let z = Number::Complex(real, imaginary);
    let Number::Complex(squared_real, squared_imaginary) = Number::Float(1.0) - Number::pow(z, Number::Float(2.0)) else {
        unreachable!()
    }; // 1 - z ^ 2
    let Number::Complex(real, imaginary) = I * z + Number::complex(sqrt(squared_real, squared_imaginary)) else {
        unreachable!()
    }; // iz + (1 - z ^ 2) ^ (1 / 2)
    let Number::Complex(real, imaginary) = Number::complex(ln(real, imaginary)) / I else {
        unreachable!()
    }; // final result
    return (real, imaginary);
}
pub fn arccosine(real: f64, imaginary: f64) -> Complex {
    // (1 / i) ln(z + i(1 - z ^ 2) ^ (1 / 2))
    let z = Number::Complex(real, imaginary);
    let Number::Complex(squared_real, squared_imaginary) = Number::Float(1.0) - Number::pow(z, Number::Float(2.0)) else {
        unreachable!()
    }; // 1 - z ^ 2
    let Number::Complex(real, imaginary) = z + I * Number::complex(sqrt(squared_real, squared_imaginary)) else {
        unreachable!()
    }; // z + i(1 - z ^ 2) ^ (1 / 2)
    let Number::Complex(real, imaginary) = Number::complex(ln(real, imaginary)) / I else {
        unreachable!()
    }; // final result
    return (real, imaginary);
}

// hyperbolic functions
pub fn sine_hyperbolic(real: f64, imaginary: f64) -> Complex {
    // sinh(x + iy) = sinh(x)cos(y) + i cosh(x)sin(y)
    let result_real = real.sinh() * imaginary.cos();
    let result_imaginary = real.cosh() * imaginary.sin();
    return (result_real, result_imaginary);
}
pub fn cosine_hyperbolic(real: f64, imaginary: f64) -> Complex {
    // sinh(x + iy) = cosh(x)cos(y) + i sinh(x)sin(y)
    let result_real = real.cosh() * imaginary.cos();
    let result_imaginary = real.sinh() * imaginary.sin();
    return (result_real, result_imaginary);
}

// --- --- --- --- --- ---

#[test]
fn phase_angle_test() {
    use std::f64::consts::FRAC_PI_4;
    let phase1 = Polar::phase_angle(1.0, 1.0);
    let phase2 = Polar::phase_angle(-1.0, 1.0);
    let phase3 = Polar::phase_angle(-1.0, -1.0);
    let phase4 = Polar::phase_angle(1.0, -1.0);

    assert_eq!(phase1, FRAC_PI_4);
    assert_eq!(phase2, 3.0 * FRAC_PI_4);
    assert_eq!(phase3, -3.0 * FRAC_PI_4);
    assert_eq!(phase4, -FRAC_PI_4);
}

#[test]
fn ln_test() {
    use std::f64::consts::FRAC_PI_4;
    assert_eq!(ln(1.0, 1.0), (0.3465735902799727, FRAC_PI_4));
}

#[test]
fn sqrt_test() {
    assert_eq!(sqrt(1.0, 1.0), (1.09868411346781, 0.45508986056222733))
}

#[test]
fn exp_test() {
    assert_eq!(exp(1.0, 1.0), (1.4686939399158851, 2.2873552871788423))
}

// trigonometric functions test
#[test]
fn sine_test() {
    assert_eq!(sine(1.0, 1.0), (1.2984575814159773, 0.6349639147847361))
}

#[test]
fn cosine_test() {
    assert_eq!(cosine(1.0, 1.0), (0.8337300251311491, -0.9888977057628651))
}

#[test]
fn arctangent_test() {
    assert_eq!(arctangent(1.0, 1.0), (1.0172219678978514, 0.40235947810852507))
}

#[test]
fn arcsine_test() {
    assert_eq!(arcsine(0.5, 0.3), (0.4930392405856184, 0.3342998177749377));
    assert_eq!(arcsine(1.0, 1.0), (0.6662394324925147, 1.061275061905036));
}

#[test]
fn arccosine_test() {
    assert_eq!(arccosine(0.5, 0.3), (1.0777570862092782, -0.33429981777493784));
    assert_eq!(arccosine(1.0, 1.0), (0.9045568943023813, -1.0612750619050357));
}

#[test]
fn sine_hyperbolic_test() {
    assert_eq!(sine_hyperbolic(1.0, 1.0), (0.6349639147847361, 1.2984575814159773))
}

#[test]
fn cosine_hyperbolic_test() {
    assert_eq!(cosine_hyperbolic(1.0, 1.0), (0.8337300251311491, 0.9888977057628651))
}

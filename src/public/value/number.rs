use std::cmp::{self, PartialEq};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::public::error::{internal_error, InternalComponent, math_error};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Number {
    NotANumber,

    Int(i64),
    Float(f64),
    Fraction(i64, i64),
}

impl Number {
    pub fn modulo(dividend: Self, divisor: Self) -> Self{
        if let (Self::NotANumber, _) | (_, Self::NotANumber) = (dividend, divisor) {
            return Self::NotANumber;
        }

        // the divisor can not be ZERO
        if divisor.float_value() == 0.0 {
            math_error("modulo by zero").unwrap_err();
            return Self::NotANumber;
        }

        if let (Self::Float(_), _) | (_, Self::Float(_)) = (dividend, divisor) {
            let f1 = dividend.float_value();
            let f2 = divisor.float_value();
            return Number::Float(f1 % f2);
        }

        match (dividend, divisor) {
            (Self::Int(i1), Self::Int(i2)) => Number::Int(i1 % i2),
            (Self::Int(i), Self::Fraction(upper, lower)) | (Self::Fraction(upper, lower), Self::Int(i)) => {
                let temp1 = i * lower;
                let temp2 = upper * lower;
                Number::Fraction(temp1 % temp2, lower)
            }
            (Self::Fraction(upper1, lower1), Self::Fraction(upper2, lower2)) => {
                let lower_lcm = Self::lcm(lower1, lower2);
                let multed_upper1 = upper1 * (lower_lcm / lower1);
                let multed_upper2 = upper2 * (lower_lcm / lower2);
                Number::Fraction(multed_upper1 % multed_upper2, lower_lcm).reduce().unwrap() 
            }
            _ => unreachable!()
        }
    }

    pub fn pow(base: Self, target: Self) -> Self {
        match base {
            Self::Int(num1) => match target {
                Self::Int(num2) => {
                    if num2 >= 0 {
                        Self::Int(num1.pow(num2 as u32))
                    } else {
                        Self::Float((num1 as f64).powi(num2 as i32))
                    }
                }
                Self::Float(num2) => Self::Float((num1 as f64).powf(num2)),
                Self::Fraction(_, _) => Self::Float((num1 as f64).powf(target.float_value())),
                _ => Self::NotANumber,
            },
            Self::Float(num1) => match target {
                Self::Int(num2) => Self::Float(num1.powi(num2 as i32)),
                Self::Float(num2) => Self::Float(num1.powf(num2)),
                Self::Fraction(_, _) => Self::Float(num1.powf(target.float_value())),
                _ => Self::NotANumber,
            },
            Self::Fraction(upper, lower) => match target {
                Self::Int(num2) => {
                    let upper_powed = upper.pow(num2 as u32);
                    let lower_powed = lower.pow(num2 as u32);
                    Self::Fraction(upper_powed, lower_powed).reduce().unwrap()
                }
                Self::Float(num2) => {
                    let f_value = base.float_value();
                    Self::Float(f_value.powf(num2))
                }
                Self::Fraction(_, _) => {
                    let self_f = base.float_value();
                    let target_f = target.float_value();
                    Self::Float(self_f.powf(target_f))
                }
                _ => Self::NotANumber,
            },
            Self::NotANumber => Self::NotANumber,
        }
    }

    pub fn not(&self) -> Self {
        match self {
            Self::Int(i) => Self::Int(!(*i > 0) as i64),
            Self::Float(f) => Self::Int(!(*f > 0.0) as i64),
            Self::Fraction(upper, _) => {
                if *upper == 0 {
                    Self::Int(1)
                } else {
                    Self::Int(0)
                }
            }
            Self::NotANumber => Self::Int(1),
        }
    }

    pub fn int(&self) -> Self {
        match self {
            Self::Float(f) => Self::Int(*f as i64),
            Self::Fraction(_, _) => Self::Int(self.int_value()),
            _ => self.clone(),
        }
    }
    pub fn float(&self) -> Self {
        match self {
            Self::Int(i) => Self::Float(*i as f64),
            Self::Fraction(_, _) => Self::Float(self.float_value()),
            _ => self.clone(),
        }
    }

    pub fn int_value(self) -> i64 {
        match self {
            Self::Int(i) => i,
            Self::Float(f) => f as i64,
            Self::Fraction(upper, lower) => {
                if lower > upper {
                    0
                } else {
                    upper / lower
                }
            }
            Self::NotANumber => 0,
        }
    }
    pub fn float_value(self) -> f64 {
        match self {
            Self::Int(i) => i as f64,
            Self::Float(f) => f,
            Self::Fraction(upper, lower) => (upper as f64) / (lower as f64),
            Self::NotANumber => 0_f64,
        }
    }

    fn float_cmp(num1: f64, num2: f64) -> bool {
        const EPS: f64 = f64::EPSILON;
        let diff = num1 - num2;
        let diff_abs = diff.abs();
        return diff_abs <= EPS;
    }

    fn reduce(&self) -> Result<Self, ()> {
        // this method is specially for Number::Fraction
        let Self::Fraction(mut upper, mut lower) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Number::reduce` invocation"
            )?);
        };
        let gcd_result = Self::gcd(upper, lower);
        upper /= gcd_result;
        lower /= gcd_result;
        return Ok(Self::Fraction(upper, lower));
    }
    // greatest common divisor
    fn gcd(n1: i64, n2: i64) -> i64 {
        if n1 == n2 {
            return n1;
        }

        // avoid negative number
        let larger = cmp::max(n1, n2).abs();
        let smaller = cmp::min(n1, n2).abs();

        if smaller == 0 {
            return larger;
        }

        return Self::gcd(smaller, larger % smaller);
    }
    // least common multiple
    fn lcm(n1: i64, n2: i64) -> i64 {
        return (n1 * n2) / Self::gcd(n1, n2);
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotANumber => write!(f, "Not a Number"),

            Self::Int(num) => write!(f, "{}", num),
            Self::Float(num) => {
                // optimize float number output
                // example:
                // 1.0000000000 -> 1
                if *num == num.floor() {
                    write!(f, "{}", *num as i64)
                } else {
                    write!(f, "{:.10}", num)
                }
            }
            Self::Fraction(upper, lower) => {
                write!(f, "({} / {})", upper, lower)
            }
        }
    }
}

// override operating symbols
impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self == Self::NotANumber || other == Self::NotANumber {
            return Self::NotANumber;
        }

        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Self::Float(f1 + f2);
        }

        match self {
            Self::Int(num1) => match other {
                Self::Int(num2) => Self::Int(num1 + num2),
                Self::Fraction(upper, lower) => Self::Fraction(upper + num1 * lower, lower)
                    .reduce()
                    .unwrap(),
                _ => unreachable!(),
            },
            Self::Fraction(upper, lower) => match other {
                Self::Int(num2) => Self::Fraction(upper + num2 * lower, lower)
                    .reduce()
                    .unwrap(),
                Self::Fraction(other_upper, other_lower) => {
                    let lower_lcm = Self::lcm(lower, other_lower);
                    let self_factor = lower_lcm / lower;
                    let other_factor = lower_lcm / other_lower;
                    let res_upper = upper * self_factor + other_upper * other_factor;
                    Self::Fraction(res_upper, lower_lcm).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self == Self::NotANumber || other == Self::NotANumber {
            return Self::NotANumber;
        }

        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Self::Float(f1 - f2);
        }

        match self {
            Self::Int(num1) => match other {
                Self::Int(num2) => Self::Int(num1 - num2),
                Self::Fraction(upper, lower) => Self::Fraction(num1 * lower - upper, lower)
                    .reduce()
                    .unwrap(),
                _ => unreachable!(),
            },
            Self::Fraction(upper, lower) => match other {
                Self::Int(num2) => Self::Fraction(upper - num2 * lower, lower)
                    .reduce()
                    .unwrap(),
                Self::Fraction(other_upper, other_lower) => {
                    let lower_lcm = Self::lcm(lower, other_lower);
                    let self_factor = lower_lcm / lower;
                    let other_factor = lower_lcm / other_lower;
                    let res_upper = upper * self_factor - other_upper * other_factor;
                    Self::Fraction(res_upper, lower_lcm).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self == Self::NotANumber || other == Self::NotANumber {
            return Self::NotANumber;
        }

        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Self::Float(f1 * f2);
        }

        match self {
            Self::Int(num1) => match other {
                Self::Int(num2) => Self::Int(num1 * num2),
                Self::Fraction(upper, lower) => {
                    Self::Fraction(num1 * upper, lower).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            Self::Fraction(upper, lower) => match other {
                Self::Int(num2) => Self::Fraction(upper * num2, lower).reduce().unwrap(),
                Self::Fraction(other_upper, other_lower) => {
                    let muled_upper = upper * other_upper;
                    let muled_lower = lower * other_lower;
                    Self::Fraction(muled_upper, muled_lower).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self == Self::NotANumber || other == Self::NotANumber {
            return Self::NotANumber;
        }

        // when the divisor is ZERO
        if other.float_value() == 0.0 {
            math_error("the divisor should not to be ZERO").unwrap_err();
            return Number::NotANumber;
        }

        // when either `self` or `other` is float
        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();

            return Self::Float(f1 / f2);
        }

        match self {
            Self::Int(num1) => match other {
                Self::Int(num2) => Self::Int(num1 / num2),
                Self::Fraction(upper, lower) => {
                    Self::Fraction(num1 * lower, upper).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            Self::Fraction(upper, lower) => match other {
                Self::Int(num2) => Self::Fraction(upper, lower * num2).reduce().unwrap(),
                Self::Fraction(other_upper, other_lower) => {
                    Self::Fraction(upper * other_lower, lower * other_upper)
                        .reduce()
                        .unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

// --- --- --- --- --- ---

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if let (Self::NotANumber, _) | (_, Self::NotANumber) = (self, other) {
            return None;
        }

        let self_f = self.float_value();
        let other_f = other.float_value();
        return self_f.partial_cmp(&other_f);
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::NotANumber, _) | (_, Self::NotANumber) = (self, other) {
            return false;
        }

        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Self::float_cmp(f1, f2);
        }

        match self {
            Self::Int(num1) => match other {
                Self::Int(num2) => *num1 == *num2,
                Self::Fraction(_, _) => *num1 as f64 == other.float_value(),
                _ => unreachable!(),
            },
            Self::Fraction(_, _) => match other {
                Self::Int(num2) => self.int_value() == *num2,
                Self::Fraction(_, _) => Self::float_cmp(self.float_value(), other.float_value()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

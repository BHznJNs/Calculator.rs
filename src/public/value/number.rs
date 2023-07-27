use std::cmp::{self, PartialEq};
use std::f64::INFINITY;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::public::error::{internal_error, InternalComponent};
use crate::utils::output::print_line__;

#[derive(PartialOrd, Clone, Copy)]
pub enum Number {
    NotANumber,

    Int(i64),
    Float(f64),
    Fraction(i64, i64),
}

impl Number {
    pub fn pow(self, target: Self) -> Self {
        match self {
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
                    let f_value = self.float_value();
                    Self::Float(f_value.powf(num2))
                }
                Self::Fraction(_, _) => {
                    Self::Float(self.float_value().powf(target.float_value()))
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
        let gcd_val = Self::gcd(upper, lower);
        upper /= gcd_val;
        lower /= gcd_val;
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

        // when either `self` or `other` is float
        if let (Self::Float(_), _) | (_, Self::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();

            if f2 == 0.0 {
                print_line__("The dividend should not to be ZERO!");
                match f1 {
                    x if x == 0.0 => return Self::Float(0.0),
                    x if x > 0.0 => return Self::Float(INFINITY),
                    x if x < 0.0 => return Self::Float(-INFINITY),
                    _ => unreachable!(),
                }
            }
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
                Self::Fraction(_, _) => {
                    Self::float_cmp(self.float_value(), other.float_value())
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

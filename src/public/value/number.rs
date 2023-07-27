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
    pub fn pow(self, target: Number) -> Number {
        match self {
            Number::Int(num1) => match target {
                Number::Int(num2) => {
                    if num2 >= 0 {
                        Number::Int(num1.pow(num2 as u32))
                    } else {
                        Number::Float((num1 as f64).powi(num2 as i32))
                    }
                }
                Number::Float(num2) => Number::Float((num1 as f64).powf(num2)),
                Number::Fraction(_, _) => Number::Float((num1 as f64).powf(target.float_value())),
                _ => Number::NotANumber,
            },
            Number::Float(num1) => match target {
                Number::Int(num2) => Number::Float(num1.powi(num2 as i32)),
                Number::Float(num2) => Number::Float(num1.powf(num2)),
                Number::Fraction(_, _) => Number::Float(num1.powf(target.float_value())),
                _ => Number::NotANumber,
            },
            Number::Fraction(upper, lower) => match target {
                Number::Int(num2) => {
                    let upper_powed = upper.pow(num2 as u32);
                    let lower_powed = lower.pow(num2 as u32);
                    Number::Fraction(upper_powed, lower_powed).reduce().unwrap()
                }
                Number::Float(num2) => {
                    let f_value = self.float_value();
                    Number::Float(f_value.powf(num2))
                }
                Number::Fraction(_, _) => {
                    Number::Float(self.float_value().powf(target.float_value()))
                }
                _ => Number::NotANumber,
            },
            Number::NotANumber => Number::NotANumber,
        }
    }

    pub fn not(&self) -> Number {
        match self {
            Number::Int(i) => Number::Int(!(*i > 0) as i64),
            Number::Float(f) => Number::Int(!(*f > 0.0) as i64),
            Number::Fraction(upper, _) => {
                if *upper == 0 {
                    Number::Int(1)
                } else {
                    Number::Int(0)
                }
            }
            Number::NotANumber => Number::Int(1),
        }
    }

    pub fn int(&self) -> Number {
        match self {
            Number::Float(f) => Number::Int(*f as i64),
            Number::Fraction(_, _) => Number::Int(self.int_value()),
            _ => self.clone(),
        }
    }
    pub fn float(&self) -> Number {
        match self {
            Number::Int(i) => Number::Float(*i as f64),
            Number::Fraction(_, _) => Number::Float(self.float_value()),
            _ => self.clone(),
        }
    }

    pub fn int_value(self) -> i64 {
        match self {
            Number::Int(i) => i,
            Number::Float(f) => f as i64,
            Number::Fraction(upper, lower) => {
                if lower > upper {
                    0
                } else {
                    upper / lower
                }
            }
            Number::NotANumber => 0,
        }
    }
    pub fn float_value(self) -> f64 {
        match self {
            Number::Int(i) => i as f64,
            Number::Float(f) => f,
            Number::Fraction(upper, lower) => (upper as f64) / (lower as f64),
            Number::NotANumber => 0_f64,
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
        let Number::Fraction(mut upper, mut lower) = self else {
            return Err(internal_error(
                InternalComponent::InternalFn,
                "invalid `Number::reduce` invocation"
            )?);
        };
        let gcd_val = Number::gcd(upper, lower);
        upper /= gcd_val;
        lower /= gcd_val;
        return Ok(Number::Fraction(upper, lower));
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

        return Number::gcd(smaller, larger % smaller);
    }
    // least common multiple
    fn lcm(n1: i64, n2: i64) -> i64 {
        return (n1 * n2) / Number::gcd(n1, n2);
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::NotANumber => write!(f, "Not a Number"),

            Number::Int(num) => write!(f, "{}", num),
            Number::Float(num) => {
                // optimize float number output
                // example:
                // 1.0000000000 -> 1
                if *num == num.floor() {
                    write!(f, "{}", *num as i64)
                } else {
                    write!(f, "{:.10}", num)
                }
            }
            Number::Fraction(upper, lower) => {
                write!(f, "({} / {})", upper, lower)
            }
        }
    }
}

// override operating symbols
impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        if self == Number::NotANumber || other == Number::NotANumber {
            return Number::NotANumber;
        }

        if let (Number::Float(_), _) | (_, Number::Float(_)) = (self, other) {
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Number::Float(f1 + f2);
        }

        match self {
            Number::Int(num1) => match other {
                Number::Int(num2) => Number::Int(num1 + num2),
                Number::Fraction(upper, lower) => Number::Fraction(upper + num1 * lower, lower)
                    .reduce()
                    .unwrap(),
                _ => unreachable!(),
            },
            Number::Fraction(upper, lower) => match other {
                Number::Int(num2) => Number::Fraction(upper + num2 * lower, lower)
                    .reduce()
                    .unwrap(),
                Number::Fraction(other_upper, other_lower) => {
                    let lower_lcm = Number::lcm(lower, other_lower);
                    let self_factor = lower_lcm / lower;
                    let other_factor = lower_lcm / other_lower;
                    let res_upper = upper * self_factor + other_upper * other_factor;
                    Number::Fraction(res_upper, lower_lcm).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        if self == Number::NotANumber || other == Number::NotANumber {
            return Number::NotANumber;
        }

        if let (Number::Float(_), _) | (_, Number::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Number::Float(f1 - f2);
        }

        match self {
            Number::Int(num1) => match other {
                Number::Int(num2) => Number::Int(num1 - num2),
                Number::Fraction(upper, lower) => Number::Fraction(num1 * lower - upper, lower)
                    .reduce()
                    .unwrap(),
                _ => unreachable!(),
            },
            Number::Fraction(upper, lower) => match other {
                Number::Int(num2) => Number::Fraction(upper - num2 * lower, lower)
                    .reduce()
                    .unwrap(),
                Number::Fraction(other_upper, other_lower) => {
                    let lower_lcm = Number::lcm(lower, other_lower);
                    let self_factor = lower_lcm / lower;
                    let other_factor = lower_lcm / other_lower;
                    let res_upper = upper * self_factor - other_upper * other_factor;
                    Number::Fraction(res_upper, lower_lcm).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        if self == Number::NotANumber || other == Number::NotANumber {
            return Number::NotANumber;
        }

        if let (Number::Float(_), _) | (_, Number::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Number::Float(f1 * f2);
        }

        match self {
            Number::Int(num1) => match other {
                Number::Int(num2) => Number::Int(num1 * num2),
                Number::Fraction(upper, lower) => {
                    Number::Fraction(num1 * upper, lower).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            Number::Fraction(upper, lower) => match other {
                Number::Int(num2) => Number::Fraction(upper * num2, lower).reduce().unwrap(),
                Number::Fraction(other_upper, other_lower) => {
                    let muled_upper = upper * other_upper;
                    let muled_lower = lower * other_lower;
                    Number::Fraction(muled_upper, muled_lower).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        if self == Number::NotANumber || other == Number::NotANumber {
            return Number::NotANumber;
        }

        // when either `self` or `other` is float
        if let (Number::Float(_), _) | (_, Number::Float(_)) = (self, other) {
            // convert num1 and num2 to float type
            let f1 = self.float_value();
            let f2 = other.float_value();

            if f2 == 0.0 {
                print_line__("The dividend should not to be ZERO!");
                match f1 {
                    x if x == 0.0 => return Number::Float(0.0),
                    x if x > 0.0 => return Number::Float(INFINITY),
                    x if x < 0.0 => return Number::Float(-INFINITY),
                    _ => unreachable!(),
                }
            }
            return Number::Float(f1 / f2);
        }

        match self {
            Number::Int(num1) => match other {
                Number::Int(num2) => Number::Int(num1 / num2),
                Number::Fraction(upper, lower) => {
                    Number::Fraction(num1 * lower, upper).reduce().unwrap()
                }
                _ => unreachable!(),
            },
            Number::Fraction(upper, lower) => match other {
                Number::Int(num2) => Number::Fraction(upper, lower * num2).reduce().unwrap(),
                Number::Fraction(other_upper, other_lower) => {
                    Number::Fraction(upper * other_lower, lower * other_upper)
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
        if let (Number::NotANumber, _) | (_, Number::NotANumber) = (self, other) {
            return false;
        }

        if let (Number::Float(_), _) | (_, Number::Float(_)) = (self, other) {
            let f1 = self.float_value();
            let f2 = other.float_value();
            return Number::float_cmp(f1, f2);
        }

        match self {
            Number::Int(num1) => match other {
                Number::Int(num2) => *num1 == *num2,
                Number::Fraction(_, _) => *num1 as f64 == other.float_value(),
                _ => unreachable!(),
            },
            Number::Fraction(_, _) => match other {
                Number::Int(num2) => self.int_value() == *num2,
                Number::Fraction(_, _) => {
                    Number::float_cmp(self.float_value(), other.float_value())
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

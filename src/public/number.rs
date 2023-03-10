use std::f64::INFINITY;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Number {
    NotANumber,

    Int(i64),
    Float(f64),
}

impl Number {
    pub fn pow(self, target: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match target {
                    Number::Int(num2) => {
                        if num2 >= 0 {
                            Number::Int(num1.pow(num2 as u32))
                        } else {
                            Number::Float((num1 as f64).powi(num2 as i32))
                        }
                    },
                    Number::Float(num2) => Number::Float((num1 as f64).powf(num2)),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::Float(num1) => {
                match target {
                    Number::Int(num2) => Number::Float(num1.powi(num2 as i32)),
                    Number::Float(num2) => Number::Float(num1.powf(num2)),
                    Number::NotANumber => Number::NotANumber,
                }
            }
            Number::NotANumber => Number::NotANumber,
        }
    }
    pub fn float(self) -> Number {
        match self {
            Number::Int(i) => Number::Float(i as f64),
            _ => self,
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        return Number::Int(0);
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::NotANumber => write!(f, "Not a Number"),
            Number::Int(num) => write!(f, "{}", num),
            Number::Float(num) => {
                if *num == num.floor() {
                    write!(f, "{}", *num as i64)
                } else {
                    write!(f, "{:.8}", num)
                }
            },
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => Number::Int(num1 + num2),
                    Number::Float(num2) => Number::Float((num1 as f64) + num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => Number::Float(num1 + (num2 as f64)),
                    Number::Float(num2) => Number::Float(num1 + num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::NotANumber => Number::NotANumber,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => Number::Int(num1 - num2),
                    Number::Float(num2) => Number::Float((num1 as f64) - num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => Number::Float(num1 - (num2 as f64)),
                    Number::Float(num2) => Number::Float(num1 - num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::NotANumber => Number::NotANumber,
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => Number::Int(num1 * num2),
                    Number::Float(num2) => Number::Float((num1 as f64) * num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => Number::Float(num1 * (num2 as f64)),
                    Number::Float(num2) => Number::Float(num1 * num2),
                    Number::NotANumber => Number::NotANumber,
                }
            },
            Number::NotANumber => Number::NotANumber,
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        let num1 = self.float();
        let num2 = other.float();
        // num1 and num2 must be float

        match num1 {
            Number::Float(num1__) => {
                match num2 {
                    Number::Float(num2__) => {
                        if num2__ == 0.0 {
                            println!("The dividend should not to be ZERO!");
                            let inf = if num1__ >= 0.0 {
                                 INFINITY
                            } else {
                                -INFINITY
                            };
                            return Number::Float(inf)
                        }
                        Number::Float(num1__ / num2__)
                    },
                    _ => Number::NotANumber,
                }
            },
            _ => Number::NotANumber,
        }
    }
}
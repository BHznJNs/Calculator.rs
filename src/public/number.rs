use std::f64::INFINITY;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(Clone, Copy)]
pub enum Number {
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
                    Number::Float(num2) => {
                        Number::Float((num1 as f64).powf(num2))
                    }
                }
            },
            Number::Float(num1) => {
                match target {
                    Number::Int(num2) => {
                        Number::Float(num1.powi(num2 as i32))
                    },
                    Number::Float(num2) => {
                        Number::Float(num1.powf(num2))
                    }
                }
            }
        }
    }
    pub fn float(self) -> Number {
        match self {
            Number::Int(num) => Number::Float(num as f64),
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
            Number::Int(num) => write!(f, "{}", num),
            Number::Float(num) => write!(f, "{:.8}", num),
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Int(num1 + num2)
                    },
                    Number::Float(num2) => {
                        Number::Float((num1 as f64) + num2)
                    }
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Float(num1 + (num2 as f64))
                    },
                    Number::Float(num2) => {
                        Number::Float(num1 + num2)
                    }
                }
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Int(num1 - num2)
                    },
                    Number::Float(num2) => {
                        Number::Float((num1 as f64) - num2)
                    }
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Float(num1 - (num2 as f64))
                    },
                    Number::Float(num2) => {
                        Number::Float(num1 - num2)
                    }
                }
            }
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        match self {
            Number::Int(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Int(num1 * num2)
                    },
                    Number::Float(num2) => {
                        Number::Float((num1 as f64) * num2)
                    }
                }
            },
            Number::Float(num1) => {
                match other {
                    Number::Int(num2) => {
                        Number::Float(num1 * (num2 as f64))
                    },
                    Number::Float(num2) => {
                        Number::Float(num1 * num2)
                    }
                }
            }
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        let num1 = self.float();
        let num2 = other.float();

        match num1 {
            Number::Int(_) => self,
            Number::Float(num1__) => {
                match num2 {
                    Number::Int(_) => other,
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
                        let result = num1__ / num2__;
                        if result == result.floor() {
                            Number::Int(result as i64)
                        } else {
                            Number::Float(result)
                        }
                    }
                }
            }
        }
    }
}
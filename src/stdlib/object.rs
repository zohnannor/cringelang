use std::fmt::{Debug, Display};

use super::{bool::Bool, number::Number};

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Number(Number),
    Bool(Bool),
}

#[allow(unused)]
impl Object {
    pub fn add(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
                (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 + b),
                (Number::Float(a), Number::Int(b)) => Number::Float(a + b as f64),
                (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n + 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Float(n + 1.0),
                (Number::Float(n), Bool::False) => Number::Float(n),
            }),
            (Object::Bool(_), Object::Number(_)) => other.add(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(1),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '+',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn sub(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a - b),
                (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 - b),
                (Number::Float(a), Number::Int(b)) => Number::Float(a - b as f64),
                (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n - 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Float(n - 1.0),
                (Number::Float(n), Bool::False) => Number::Float(n),
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(0),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(-1),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (Object::Bool(_), Object::Number(_)) => other.sub(self)?,
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '-',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn mul(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a * b),
                (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 * b),
                (Number::Float(a), Number::Int(b)) => Number::Float(a * b as f64),
                (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n),
                (Number::Int(n), Bool::False) => Number::Int(0),
                (Number::Float(n), Bool::True) => Number::Float(n),
                (Number::Float(n), Bool::False) => Number::Float(0.0),
            }),
            (Object::Bool(_), Object::Number(_)) => other.mul(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Int(0),
                (Bool::False, Bool::True) => Number::Int(0),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '*',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn div(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Float(a as f64 / b as f64),
                (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 / b),
                (Number::Float(a), Number::Int(b)) => Number::Float(a / b as f64),
                (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n),
                (Number::Int(n), Bool::False) => Number::Float(f64::INFINITY),
                (Number::Float(n), Bool::True) => Number::Float(n),
                (Number::Float(n), Bool::False) => Number::Float(f64::INFINITY),
            }),
            (Object::Bool(_), Object::Number(_)) => other.div(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Float(f64::INFINITY),
                (Bool::False, Bool::True) => Number::Float(0.0),
                (Bool::False, Bool::False) => Number::Float(f64::INFINITY),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '/',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn r#mod(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (_, Number::Int(0)) => Number::Float(f64::NAN),
                (_, Number::Float(n)) if n == 0.0 => Number::Float(f64::NAN),
                (Number::Int(a), Number::Int(b)) => Number::Int(a % b),
                (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 % b),
                (Number::Float(a), Number::Int(b)) => Number::Float(a % b as f64),
                (Number::Float(a), Number::Float(b)) => Number::Float(a % b),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (_, Bool::True) => Number::Int(0),
                (_, Bool::False) => Number::Float(f64::NAN),
            }),
            (Object::Bool(_), Object::Number(_)) => other.r#mod(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (_, Bool::True) => Number::Int(0),
                (_, Bool::False) => Number::Float(f64::NAN),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '%',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn pow(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(0), Number::Int(0)) => Number::Float(f64::NAN),
                (Number::Int(0), Number::Float(n)) if n == 0.0 => Number::Float(f64::NAN),
                (Number::Float(n), Number::Int(0)) if n == 0.0 => Number::Float(f64::NAN),
                (Number::Float(n), Number::Float(m)) if n == 0.0 && m == 0.0 => {
                    Number::Float(f64::NAN)
                }
                (Number::Int(a), Number::Int(b)) => Number::Float((a as f64).powf(b as f64)),
                (Number::Int(a), Number::Float(b)) => Number::Float((a as f64).powf(b)),
                (Number::Float(a), Number::Int(b)) => Number::Float(a.powf(b as f64)),
                (Number::Float(a), Number::Float(b)) => Number::Float(a.powf(b)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(0), Bool::False) => Number::Float(f64::NAN),
                (Number::Float(n), Bool::False) if n == 0.0 => Number::Float(f64::NAN),
                (Number::Int(n), Bool::True) => Number::Int(n),
                (Number::Int(_), Bool::False) => Number::Int(1),
                (Number::Float(n), Bool::True) => Number::Float(n),
                (Number::Float(_), Bool::False) => Number::Int(1),
            }),
            (Object::Bool(_), Object::Number(_)) => other.pow(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(0),
                (Bool::False, Bool::False) => Number::Float(f64::NAN),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    "**",
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }

    pub fn not(&self) -> Result<Object, String> {
        Ok(Object::Bool(match self {
            Object::Number(n) => match *n {
                Number::Int(n) => {
                    if n == 0i128 {
                        Bool::True
                    } else {
                        Bool::False
                    }
                }
                Number::Float(n) => {
                    if n == 0.0 {
                        Bool::True
                    } else {
                        Bool::False
                    }
                }
            },
            Object::Bool(b) => match b {
                Bool::True => Bool::False,
                Bool::False => Bool::True,
            },
            _ => {
                return Err(format!(
                    "Operator '{}' is not supported for type {}",
                    '!',
                    self.class_name()
                ))
            }
        }))
    }
    pub fn orb(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a | b),
                (Number::Int(a), Number::Float(b)) => Number::Int(a | (b as i128)),
                (Number::Float(a), Number::Int(b)) => Number::Int((a as i128) | b),
                (Number::Float(a), Number::Float(b)) => Number::Int((a as i128) | (b as i128)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n | 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Int((n as i128) | 1),
                (Number::Float(n), Bool::False) => Number::Int(n as i128),
            }),
            (Object::Bool(_), Object::Number(_)) => other.orb(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(1),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '|',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn andb(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a & b),
                (Number::Int(a), Number::Float(b)) => Number::Int(a & (b as i128)),
                (Number::Float(a), Number::Int(b)) => Number::Int((a as i128) & b),
                (Number::Float(a), Number::Float(b)) => Number::Int((a as i128) & (b as i128)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n & 1),
                (Number::Int(_), Bool::False) => Number::Int(0),
                (Number::Float(n), Bool::True) => Number::Int((n as i128) & 1),
                (Number::Float(_), Bool::False) => Number::Int(0),
            }),
            (Object::Bool(_), Object::Number(_)) => other.andb(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(1),
                (Bool::True, Bool::False) => Number::Int(0),
                (Bool::False, Bool::True) => Number::Int(0),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '&',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn xor(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a ^ b),
                (Number::Int(a), Number::Float(b)) => Number::Int(a ^ (b as i128)),
                (Number::Float(a), Number::Int(b)) => Number::Int((a as i128) ^ b),
                (Number::Float(a), Number::Float(b)) => Number::Int((a as i128) ^ (b as i128)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n ^ 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Int((n as i128) ^ 1),
                (Number::Float(n), Bool::False) => Number::Int(n as i128),
            }),
            (Object::Bool(_), Object::Number(_)) => other.xor(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(0),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(1),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '^',
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn rsh(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a >> b),
                (Number::Int(a), Number::Float(b)) => Number::Int(a >> (b as i128)),
                (Number::Float(a), Number::Int(b)) => Number::Int((a as i128) >> b),
                (Number::Float(a), Number::Float(b)) => Number::Int((a as i128) >> (b as i128)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n >> 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Int((n as i128) >> 1),
                (Number::Float(n), Bool::False) => Number::Int(n as i128),
            }),
            (Object::Bool(_), Object::Number(_)) => other.rsh(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(0),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(0),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    ">>",
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }
    pub fn lsh(&self, other: &Object) -> Result<Object, String> {
        Ok(match (self, other) {
            (Object::Number(a), Object::Number(b)) => Object::Number(match (*a, *b) {
                (Number::Int(a), Number::Int(b)) => Number::Int(a << b),
                (Number::Int(a), Number::Float(b)) => Number::Int(a << (b as i128)),
                (Number::Float(a), Number::Int(b)) => Number::Int((a as i128) << b),
                (Number::Float(a), Number::Float(b)) => Number::Int((a as i128) << (b as i128)),
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (Number::Int(n), Bool::True) => Number::Int(n << 1),
                (Number::Int(n), Bool::False) => Number::Int(n),
                (Number::Float(n), Bool::True) => Number::Int((n as i128) << 1),
                (Number::Float(n), Bool::False) => Number::Int(n as i128),
            }),
            (Object::Bool(_), Object::Number(_)) => other.rsh(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (Bool::True, Bool::True) => Number::Int(2),
                (Bool::True, Bool::False) => Number::Int(1),
                (Bool::False, Bool::True) => Number::Int(0),
                (Bool::False, Bool::False) => Number::Int(0),
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    "<<",
                    a.class_name(),
                    b.class_name()
                ))
            }
        })
    }

    pub fn eq(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if match (*a, *b) {
                    (Number::Int(a), Number::Int(b)) => a == b,
                    (Number::Int(a), Number::Float(b)) => (a as f64) == b,
                    (Number::Float(a), Number::Int(b)) => a == (b as f64),
                    (Number::Float(a), Number::Float(b)) => a == b,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Number(_), Object::Bool(_)) => Bool::False,
            (Object::Bool(_), Object::Number(_)) => Bool::False,
            (Object::Bool(b1), Object::Bool(b2)) => match (b1, b2) {
                (Bool::True, Bool::True) => Bool::True,
                (Bool::True, Bool::False) => Bool::False,
                (Bool::False, Bool::True) => Bool::False,
                (Bool::False, Bool::False) => Bool::True,
            },
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported between types {} and {}",
                    "==",
                    self.class_name(),
                    other.class_name()
                ))
            }
        }))
    }
    pub fn ne(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(
            if let Object::Bool(Bool::True) = self.eq(other).map_err(|_err| {
                format!(
                    "Operator '{}' is not supported between types {} and {}",
                    "!=",
                    self.class_name(),
                    other.class_name()
                )
            })? {
                Bool::False
            } else {
                Bool::True
            },
        ))
    }
    pub fn gt(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if match (*a, *b) {
                    (Number::Int(a), Number::Int(b)) => a > b,
                    (Number::Int(a), Number::Float(b)) => (a as f64) > b,
                    (Number::Float(a), Number::Int(b)) => a > (b as f64),
                    (Number::Float(a), Number::Float(b)) => a > b,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Number(n), Object::Bool(b)) => {
                if match (*n, b) {
                    (Number::Int(n), Bool::True) => n > 1,
                    (Number::Int(n), Bool::False) => n > 0,
                    (Number::Float(n), Bool::True) => n > 1.0,
                    (Number::Float(n), Bool::False) => n > 0.0,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Bool(_), Object::Number(_)) => return other.lt(self),
            (Object::Bool(b1), Object::Bool(b2)) => match (b1, b2) {
                (Bool::True, Bool::True) => Bool::False,
                (Bool::True, Bool::False) => Bool::True,
                (Bool::False, Bool::True) => Bool::False,
                (Bool::False, Bool::False) => Bool::False,
            },
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported between types {} and {}",
                    '>',
                    self.class_name(),
                    other.class_name()
                ))
            }
        }))
    }
    pub fn lt(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if match (*a, *b) {
                    (Number::Int(a), Number::Int(b)) => a < b,
                    (Number::Int(a), Number::Float(b)) => (a as f64) < b,
                    (Number::Float(a), Number::Int(b)) => a < (b as f64),
                    (Number::Float(a), Number::Float(b)) => a < b,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Number(n), Object::Bool(b)) => {
                if match (*n, b) {
                    (Number::Int(n), Bool::True) => n < 1,
                    (Number::Int(n), Bool::False) => n < 0,
                    (Number::Float(n), Bool::True) => n < 1.0,
                    (Number::Float(n), Bool::False) => n < 0.0,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Bool(_), Object::Number(_)) => return other.gt(self),
            (Object::Bool(b1), Object::Bool(b2)) => match (b1, b2) {
                (Bool::True, Bool::True) => Bool::False,
                (Bool::True, Bool::False) => Bool::False,
                (Bool::False, Bool::True) => Bool::True,
                (Bool::False, Bool::False) => Bool::False,
            },
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported between types {} and {}",
                    '<',
                    self.class_name(),
                    other.class_name()
                ))
            }
        }))
    }
    pub fn gte(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if match (*a, *b) {
                    (Number::Int(a), Number::Int(b)) => a >= b,
                    (Number::Int(a), Number::Float(b)) => (a as f64) >= b,
                    (Number::Float(a), Number::Int(b)) => a >= (b as f64),
                    (Number::Float(a), Number::Float(b)) => a >= b,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Number(n), Object::Bool(b)) => {
                if match (*n, b) {
                    (Number::Int(n), Bool::True) => n >= 1,
                    (Number::Int(n), Bool::False) => n >= 0,
                    (Number::Float(n), Bool::True) => n >= 1.0,
                    (Number::Float(n), Bool::False) => n >= 0.0,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Bool(_), Object::Number(_)) => return other.lte(self),
            (Object::Bool(b1), Object::Bool(b2)) => match (b1, b2) {
                (Bool::True, Bool::True) => Bool::True,
                (Bool::True, Bool::False) => Bool::True,
                (Bool::False, Bool::True) => Bool::False,
                (Bool::False, Bool::False) => Bool::True,
            },
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported between types {} and {}",
                    ">=",
                    self.class_name(),
                    other.class_name()
                ))
            }
        }))
    }
    pub fn lte(&self, other: &Object) -> Result<Object, String> {
        Ok(Object::Bool(match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if match (*a, *b) {
                    (Number::Int(a), Number::Int(b)) => a <= b,
                    (Number::Int(a), Number::Float(b)) => (a as f64) <= b,
                    (Number::Float(a), Number::Int(b)) => a <= (b as f64),
                    (Number::Float(a), Number::Float(b)) => a <= b,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Number(n), Object::Bool(b)) => {
                if match (*n, b) {
                    (Number::Int(n), Bool::True) => n <= 1,
                    (Number::Int(n), Bool::False) => n <= 0,
                    (Number::Float(n), Bool::True) => n <= 1.0,
                    (Number::Float(n), Bool::False) => n <= 0.0,
                } {
                    Bool::True
                } else {
                    Bool::False
                }
            }
            (Object::Bool(_), Object::Number(_)) => return other.gte(self),
            (Object::Bool(b1), Object::Bool(b2)) => match (b1, b2) {
                (Bool::True, Bool::True) => Bool::True,
                (Bool::True, Bool::False) => Bool::False,
                (Bool::False, Bool::True) => Bool::True,
                (Bool::False, Bool::False) => Bool::True,
            },
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported between types {} and {}",
                    "<=",
                    self.class_name(),
                    other.class_name()
                ))
            }
        }))
    }

    pub fn idx(&self, idx: isize) -> Result<Object, String> {
        Err(format!(
            "Object of type {} can not be indexed",
            self.class_name(),
        ))
    }

    // pub fn call(&self, params, body) -> Result<Object, String> {
    //     Err(format!(
    //         "Object of type {} is not callable",
    //         self.class_name(),
    //     ))
    // }

    pub fn bool(&self) -> Bool {
        match self {
            Object::Number(n) => {
                if match *n {
                    Number::Int(n) => n == 0,
                    Number::Float(n) => n == 0.0,
                } {
                    Bool::False
                } else {
                    Bool::True
                }
            }
            Object::Bool(b) => *b,
        }
    }

    pub fn num(&self) -> Result<Number, String> {
        Err(format!(
            "{} can not be converted to Number",
            self.class_name()
        ))
    }
    pub fn str(&self) -> String {
        self.repr()
    }

    pub fn repr(&self) -> String {
        match self {
            Object::Number(n) => match n {
                Number::Int(n) => n.to_string(),
                Number::Float(n) => n.to_string(),
            },
            Object::Bool(b) => b.to_string(),
            _ => format!(
                "<{} object at {:?}>",
                self.class_name(),
                self as *const Self
            ),
        }
    }

    fn class_name(&self) -> String {
        match self {
            Object::Number(_) => "Number".to_string(),
            Object::Bool(_) => "Bool".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i128),
    Float(f64),
}

impl Number {
    pub fn add(self, other: Number) -> Number {
        match self {
            Number::Int(a) => match other {
                Number::Int(b) => Number::Int(a + b),
                Number::Float(b) => Number::Float(a as f64 + b),
            },
            Number::Float(a) => match other {
                Number::Int(b) => Number::Float(a + b as f64),
                Number::Float(b) => Number::Float(a + b),
            },
        }
    }
    pub fn sub(self, other: Number) -> Number {
        match self {
            Number::Int(a) => match other {
                Number::Int(b) => Number::Int(a - b),
                Number::Float(b) => Number::Float(a as f64 - b),
            },
            Number::Float(a) => match other {
                Number::Int(b) => Number::Float(a - b as f64),
                Number::Float(b) => Number::Float(a - b),
            },
        }
    }
    pub fn mul(self, other: Number) -> Number {
        match self {
            Number::Int(a) => match other {
                Number::Int(b) => Number::Int(a * b),
                Number::Float(b) => Number::Float(a as f64 * b),
            },
            Number::Float(a) => match other {
                Number::Int(b) => Number::Float(a * b as f64),
                Number::Float(b) => Number::Float(a * b),
            },
        }
    }

    pub fn div(self, other: Number) -> Number {
        match self {
            Number::Int(a) => match other {
                Number::Int(0) => Number::Float(f64::INFINITY),
                Number::Float(n) if n == 0.0 => Number::Float(f64::INFINITY),
                Number::Int(b) => Number::Float(a as f64 / b as f64),
                Number::Float(b) => Number::Float(a as f64 / b),
            },
            Number::Float(a) => match other {
                Number::Int(0) => Number::Float(f64::INFINITY),
                Number::Float(n) if n == 0.0 => Number::Float(f64::INFINITY),
                Number::Int(b) => Number::Float(a / b as f64),
                Number::Float(b) => Number::Float(a / b),
            },
        }
    }

    pub fn r#mod(self, other: Number) -> Result<Number, String> {
        Ok(match self {
            Number::Int(a) => match other {
                Number::Int(0) => return Err(format!("Modulo division by zero")),
                Number::Float(n) if n == 0.0 => return Err(format!("Modulo division by zero")),
                Number::Int(b) => Number::Int(a % b),
                Number::Float(b) => Number::Float(a as f64 % b),
            },
            Number::Float(a) => match other {
                Number::Int(0) => return Err(format!("Modulo division by zero")),
                Number::Float(n) if n == 0.0 => return Err(format!("Modulo division by zero")),
                Number::Int(b) => Number::Float(a % b as f64),
                Number::Float(b) => Number::Float(a % b),
            },
        })
    }

    pub fn pow(self, other: Number) -> Result<Number, String> {
        Ok(match self {
            Number::Int(a) => match other {
                Number::Int(b) => Number::Float((a as f64).powf(b as f64)),
                Number::Float(b) => Number::Float((a as f64).powf(b)),
            },
            Number::Float(a) => match other {
                Number::Int(b) => Number::Float((a).powf(b as f64)),
                Number::Float(b) => Number::Float((a).powf(b)),
            },
        })
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Number::Int(n) => n.to_string(),
                Number::Float(n) => n.to_string(),
            }
        ))
    }
}

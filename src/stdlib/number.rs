use std::{fmt::Display, rc::Rc};

use super::{bool::Bool, object::Object};

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i128),
    Float(f64),
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

impl Object for Number {
    fn bool(&self) -> Bool {
        todo!()
    }

    fn num(&self) -> Result<Number, String> {
        Ok(*self)
    }

    fn str(&self) -> String {
        match self {
            Number::Int(n) => n.to_string(),
            Number::Float(n) => n.to_string(),
        }
    }

    fn repr(&self) -> String {
        match self {
            Number::Int(n) => n.to_string(),
            Number::Float(n) => n.to_string(),
        }
    }
}

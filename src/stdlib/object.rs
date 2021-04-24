use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use super::{bool::Bool, number::Number};

pub trait ClassName {
    fn class_name(&self) -> String;
}

macro_rules! impl_class_name_and_repr {
    ($t:ty) => {
        impl ClassName for $t {
            fn class_name(&self) -> String {
                stringify!($t).to_string()
            }
        }
    };
}

impl_class_name_and_repr!(Number);
impl_class_name_and_repr!(Bool);

pub trait Object: ClassName {
    fn add(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '+',
            self.class_name(),
            other.class_name()
        ))
    }
    fn sub(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '-',
            self.class_name(),
            other.class_name()
        ))
    }
    fn mul(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '*',
            self.class_name(),
            other.class_name()
        ))
    }
    fn div(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '/',
            self.class_name(),
            other.class_name()
        ))
    }
    fn r#mod(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '%',
            self.class_name(),
            other.class_name()
        ))
    }
    fn pow(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            "**",
            self.class_name(),
            other.class_name()
        ))
    }

    fn not(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '!',
            self.class_name(),
            other.class_name()
        ))
    }
    fn orb(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '|',
            self.class_name(),
            other.class_name()
        ))
    }
    fn andb(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '&',
            self.class_name(),
            other.class_name()
        ))
    }
    fn xor(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '^',
            self.class_name(),
            other.class_name()
        ))
    }
    fn rsh(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            ">>",
            self.class_name(),
            other.class_name()
        ))
    }
    fn lsh(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            "<<",
            self.class_name(),
            other.class_name()
        ))
    }

    fn eq(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            "==",
            self.class_name(),
            other.class_name()
        ))
    }
    fn ne(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            "!=",
            self.class_name(),
            other.class_name()
        ))
    }
    fn gt(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '>',
            self.class_name(),
            other.class_name()
        ))
    }
    fn lt(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            '<',
            self.class_name(),
            other.class_name()
        ))
    }
    fn gte(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            ">=",
            self.class_name(),
            other.class_name()
        ))
    }
    fn lte(&self, other: Rc<dyn Object>) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Operator '{}' is not supported between types {} and {}",
            "<=",
            self.class_name(),
            other.class_name()
        ))
    }

    fn idx(&self, idx: isize) -> Result<Rc<dyn Object>, String> {
        Err(format!(
            "Object of type {} can not be indexed",
            self.class_name(),
        ))
    }

    // fn call(&self, params, body) -> Result<Rc<dyn Object>, String> {
    //     Err(format!(
    //         "Object of type {} is not callable",
    //         self.class_name(),
    //     ))
    // }

    fn bool(&self) -> Bool;
    fn num(&self) -> Result<Number, String> {
        Err(format!(
            "{} can not be converted to Number",
            self.class_name()
        ))
    }
    fn str(&self) -> String {
        self.repr()
    }

    fn repr(&self) -> String {
        format!(
            "<{} object at {:?}>",
            self.class_name(),
            self as *const Self
        )
    }
}

impl Debug for dyn Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<object {}>", self.repr())
    }
}

impl Display for dyn Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

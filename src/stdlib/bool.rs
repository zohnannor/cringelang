use super::{number::Number, object::Object};

#[derive(Debug, Clone, Copy)]
pub enum Bool {
    True,
    False,
}

impl Object for Bool {
    fn repr(&self) -> String {
        match self {
            Bool::True => true.to_string(),
            Bool::False => false.to_string(),
        }
    }

    fn bool(&self) -> Bool {
        *self
    }
}

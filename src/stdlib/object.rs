use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub},
    rc::Rc,
};

use super::ops::{Pow, TypeOf};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Bool(bool),
    String(Rc<RefCell<String>>),
    Char(char),
}

impl<'a> Add<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Object::Number(a + b),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => n + 1.0,
                false => *n,
            }),
            (Object::Bool(_), Object::Number(_)) => rhs.add(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 2.0,
                (true, false) => 1.0,
                (false, true) => 1.0,
                (false, false) => 0.0,
            }),
            (Object::Number(n), Object::String(s)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", n, s.borrow()))))
            }
            (Object::Bool(b), Object::String(s)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", b, s.borrow()))))
            }
            (Object::String(s1), Object::String(s2)) => Object::String(Rc::new(RefCell::new(
                format!("{}{}", s1.borrow(), s2.borrow()),
            ))),
            (Object::Char(c), Object::String(s)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", c, s.borrow()))))
            }
            (Object::String(s), Object::Number(n)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", s.borrow(), n))))
            }
            (Object::String(s), Object::Bool(b)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", s.borrow(), b))))
            }
            (Object::String(s), Object::Char(c)) => {
                Object::String(Rc::new(RefCell::new(format!("{}{}", s.borrow(), c))))
            }
            (Object::Char(c1), Object::Char(c2)) => Object::Char((*c1 as u8 + *c2 as u8) as char),
            (Object::Number(n), Object::Char(c)) => Object::Char((*n as u8 + *c as u8) as char),

            (Object::Char(_), Object::Number(_)) => rhs.add(self)?,
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '+',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Sub<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Object::Number(a - b),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => n - 1.0,
                false => *n,
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 0.0,
                (true, false) => 1.0,
                (false, true) => -1.0,
                (false, false) => 0.0,
            }),
            (Object::Bool(b), Object::Number(n)) => Object::Number(match b {
                true => 1.0 - n,
                false => -n,
            }),
            (Object::String(_), Object::String(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::Number(_), Object::String(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::Bool(_), Object::String(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::Char(_), Object::String(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::String(_), Object::Number(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::String(_), Object::Bool(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::String(_), Object::Char(_)) => {
                Object::Number(self.num()?).sub(&Object::Number(rhs.num()?))?
            }
            (Object::Char(c1), Object::Char(c2)) => Object::Char((*c1 as u8 - *c2 as u8) as char),
            (Object::Number(n), Object::Char(c)) => {
                Object::Char((*n as u8).wrapping_sub(*c as u8) as char)
            }
            (Object::Char(c), Object::Number(n)) => {
                Object::Char((*c as u8).wrapping_sub(*n as u8) as char)
            }
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '-',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Mul<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn mul(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Object::Number(a * b),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => *n,
                false => 0.0,
            }),
            (Object::Bool(_), Object::Number(_)) => rhs.mul(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 1.0,
                (true, false) => 0.0,
                (false, true) => 0.0,
                (false, false) => 0.0,
            }),
            (Object::Number(n), Object::String(s)) => {
                if n.fract() == 0.0 {
                    Object::String(Rc::new(RefCell::new(if n.is_sign_positive() {
                        s.borrow().repeat(*n as usize)
                    } else {
                        "".to_string()
                    })))
                } else {
                    return Err("Can't repeat string fractional number of times".to_string());
                }
            }
            (Object::String(_), Object::Number(_)) => rhs.mul(self)?,
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '*',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Div<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn div(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Object::Number(a / b),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => *n,
                false => f64::INFINITY,
            }),
            (Object::Bool(b), Object::Number(n)) => Object::Number(match b {
                true => n.recip(),
                false => 0.0,
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 1.0,
                (true, false) => f64::INFINITY,
                (false, true) => 0.0,
                (false, false) => f64::INFINITY,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '/',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Pow<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn pow(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(n), Object::Number(m)) => Object::Number(if *n == 0.0 && *m == 0.0 {
                f64::NAN
            } else {
                n.powf(*m)
            }),
            (Object::Number(n), Object::Bool(b)) => Object::Number(match (*n, *b) {
                (n, false) if n == 0.0 => f64::NAN,
                (_, false) => 1.0,
                (n, true) => n,
            }),
            (Object::Bool(b), Object::Number(n)) => Object::Number(match (*b, *n) {
                (false, n) if n == 0.0 => f64::NAN,
                (false, _) => 0.0,
                (true, _) => 1.0,
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 1.0,
                (true, false) => 1.0,
                (false, true) => 0.0,
                (false, false) => f64::NAN,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    "**",
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Rem<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn rem(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => Object::Number(a % b),
            (Object::Number(_), Object::Bool(b)) => Object::Number(match b {
                true => 0.0,
                false => f64::NAN,
            }),
            (Object::Bool(b), Object::Number(_)) => Object::Number(match b {
                true => 0.0,
                false => 1.0,
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (_, true) => 0.0,
                (_, false) => f64::NAN,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '%',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Not for &'a Object {
    type Output = Result<Object, String>;

    fn not(self) -> Self::Output {
        Ok(Object::Bool(match self {
            Object::Number(n) => *n == 0.0 || n.is_nan(),
            Object::Bool(b) => !b,
            _ => {
                return Err(format!(
                    "Operator '{}' is not supported for type {}",
                    '!',
                    self.r#typeof()
                ))
            }
        }))
    }
}

impl<'a> BitOr<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn bitor(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(((*a as i64) | (*b as i64)) as f64)
            }
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => ((*n as i64) | 1) as f64,
                false => *n,
            }),
            (Object::Bool(_), Object::Number(_)) => rhs.bitor(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 1.0,
                (true, false) => 1.0,
                (false, true) => 1.0,
                (false, false) => 0.0,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '|',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> BitAnd<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn bitand(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(((*a as i64) & (*b as i64)) as f64)
            }
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => ((*n as i64) & 1) as f64,
                false => 0.0,
            }),
            (Object::Bool(_), Object::Number(_)) => rhs.bitand(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 1.0,
                (true, false) => 0.0,
                (false, true) => 0.0,
                (false, false) => 0.0,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '&',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> BitXor<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn bitxor(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(((*a as i64) ^ (*b as i64)) as f64)
            }
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => ((*n as i64) ^ 1) as f64,
                false => *n,
            }),
            (Object::Bool(_), Object::Number(_)) => rhs.bitxor(self)?,
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 0.0,
                (true, false) => 1.0,
                (false, true) => 1.0,
                (false, false) => 0.0,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    '^',
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Shr<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn shr(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(((*a as i64).wrapping_shr(*b as u32)) as f64)
            }
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => ((*n as i64).wrapping_shr(1)) as f64,
                false => *n,
            }),
            (Object::Bool(_), Object::Number(_)) => Object::Number(0.0),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 0.0,
                (true, false) => 1.0,
                (false, true) => 0.0,
                (false, false) => 0.0,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    ">>",
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl<'a> Shl<&'a Object> for &'a Object {
    type Output = Result<Object, String>;

    fn shl(self, rhs: &'a Object) -> Self::Output {
        Ok(match (self, rhs) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(((*a as i64).wrapping_shl(*b as u32)) as f64)
            }
            (Object::Number(n), Object::Bool(b)) => Object::Number(match b {
                true => ((*n as i64).wrapping_shl(1)) as f64,
                false => *n,
            }),
            (Object::Bool(b), Object::Number(n)) => Object::Number(match b {
                true => (1_i32.wrapping_shl(*n as u32)) as f64,
                false => 0.0,
            }),
            (Object::Bool(b1), Object::Bool(b2)) => Object::Number(match (b1, b2) {
                (true, true) => 2.0,
                (true, false) => 1.0,
                (false, true) => 0.0,
                (false, false) => 0.0,
            }),
            (a, b) => {
                return Err(format!(
                    "Operator '{}' is not supported for types {} and {}",
                    "<<",
                    a.r#typeof(),
                    b.r#typeof()
                ))
            }
        })
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => (*a).eq(b),
            (Object::Bool(b1), Object::Bool(b2)) => b1.eq(b2),
            (Object::String(s1), Object::String(s2)) => s1.borrow().eq(&*s2.borrow()),
            (Object::Char(c1), Object::Char(c2)) => c1.eq(c2),
            _ => false,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a.partial_cmp(b),
            (Object::Bool(b), Object::Number(n)) => match (*b, n) {
                (true, n) => 1.0.partial_cmp(n),
                (false, n) => 0.0.partial_cmp(n),
            },
            (Object::Number(_), Object::Bool(_)) => other.partial_cmp(self),
            (Object::Bool(b1), Object::Bool(b2)) => b1.partial_cmp(b2),
            (Object::String(s1), Object::String(s2)) => s1.borrow().partial_cmp(&*s2.borrow()),
            (Object::Char(c1), Object::Char(c2)) => c1.partial_cmp(c2),
            _ => None,
        }
    }
}

// #[allow(unused)]
impl Object {
    pub fn eq(&self, other: &Object) -> Object {
        Object::Bool(self == other)
    }
    pub fn ne(&self, other: &Object) -> Object {
        Object::Bool(self != other)
    }

    pub fn gt(&self, other: &Object) -> Object {
        Object::Bool(self > other)
    }
    pub fn lt(&self, other: &Object) -> Object {
        Object::Bool(self < other)
    }
    pub fn gte(&self, other: &Object) -> Object {
        Object::Bool(self >= other)
    }
    pub fn lte(&self, other: &Object) -> Object {
        Object::Bool(self <= other)
    }

    #[allow(unused_variables)]
    pub fn idx(&self, idx: isize) -> Result<Object, String> {
        Err(format!(
            "Object of type {} can not be indexed",
            self.r#typeof(),
        ))
    }

    // pub fn call(&self, params, body) -> Result<Object, String> {
    //     Err(format!(
    //         "Object of type {} is not callable",
    //         self.r#typeof(),
    //     ))
    // }

    pub fn bool(&self) -> bool {
        match self {
            Object::Number(n) => *n != 0.0,
            Object::Bool(b) => *b,
            Object::String(s) => s.borrow().is_empty(),
            Object::Char(c) => *c != '\0',
        }
    }

    pub fn num(&self) -> Result<f64, String> {
        Ok(match self {
            Object::Number(n) => *n,
            Object::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Object::String(s) => s.borrow().parse::<f64>().unwrap_or(f64::NAN),
            Object::Char(c) => *c as i64 as f64,
            _ => {
                return Err(format!(
                    "{} can not be converted to Number",
                    self.r#typeof()
                ))
            }
        })
    }

    pub fn str(&self) -> String {
        self.repr()
    }

    pub fn repr(&self) -> String {
        match self {
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::Char(c) => format!("{:?}", c),
            Object::String(s) => format!("{:?}", s.borrow()),
            _ => format!("<{} object at {:?}>", self.r#typeof(), self as *const Self),
        }
    }
}

impl TypeOf for &Object {
    fn r#typeof(self) -> String {
        match self {
            Object::Number(_) => "Number".to_string(),
            Object::Bool(_) => "Bool".to_string(),
            Object::String(_) => "String".to_string(),
            Object::Char(_) => "Char".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

use std::collections::HashMap;

use super::parser::AstNode;
use super::tokens::Operator::*;
use crate::stdlib::{number::Number, object::Object};

#[derive(Default, Debug)]
pub struct Context {
    symbols: HashMap<String, Object>,
    parent: Option<Box<Context>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get(&self, var_name: &str) -> Option<Object> {
        Some(match self.symbols.get(var_name) {
            Some(value) => *value,
            None => self.parent.as_ref()?.get(var_name)?,
        })
    }

    fn set(&mut self, var_name: String, value: &Object) {
        self.symbols.insert(var_name, *value);
    }
}

pub fn visit(node: AstNode, context: &mut Context) -> Result<Object, String> {
    Ok(match node {
        AstNode::Number(n) => Object::Number(n),
        AstNode::Bool(b) => Object::Bool(b),
        AstNode::UnOp(op, node) => match op {
            Plus => visit(*node, context)?,
            Minus => visit(*node, context)?.mul(&Object::Number(Number::Int(-1)))?,
            Exclamation => visit(*node, context)?.not()?,
            _ => todo!(),
        },
        AstNode::BinOp(left_node, op, right_node) => match op {
            Plus => visit(*left_node, context)?.add(&visit(*right_node, context)?)?,
            Minus => visit(*left_node, context)?.sub(&visit(*right_node, context)?)?,
            Star => visit(*left_node, context)?.mul(&visit(*right_node, context)?)?,
            Slash => visit(*left_node, context)?.div(&visit(*right_node, context)?)?,
            StarStar => visit(*left_node, context)?.pow(&visit(*right_node, context)?)?,
            Percent => visit(*left_node, context)?.r#mod(&visit(*right_node, context)?)?,
            Pipe => visit(*left_node, context)?.orb(&visit(*right_node, context)?)?,
            Ampersand => visit(*left_node, context)?.andb(&visit(*right_node, context)?)?,
            Caret => visit(*left_node, context)?.xor(&visit(*right_node, context)?)?,
            Greater => visit(*left_node, context)?.gt(&visit(*right_node, context)?)?,
            Less => visit(*left_node, context)?.lt(&visit(*right_node, context)?)?,
            GreaterGreater => visit(*left_node, context)?.rsh(&visit(*right_node, context)?)?,
            LessLess => visit(*left_node, context)?.lsh(&visit(*right_node, context)?)?,
            GreaterEquals => visit(*left_node, context)?.gte(&visit(*right_node, context)?)?,
            LessEquals => visit(*left_node, context)?.lte(&visit(*right_node, context)?)?,
            EqualsEquals => visit(*left_node, context)?.eq(&visit(*right_node, context)?)?,
            ExclamationEquals => visit(*left_node, context)?.ne(&visit(*right_node, context)?)?,
            _ => todo!(),
        },
        AstNode::VarCreate(var_name, node) => {
            let val = visit(*node, context)?;
            context.set(var_name, &val);
            val
        }
        AstNode::VarAssign(var_name, node) => {
            if context.symbols.contains_key(&var_name) {
                let val = visit(*node, context)?;
                context.set(var_name, &val);
                val
            } else {
                return Err(format!("Name '{}' is not defined", var_name));
            }
        }
        AstNode::VarAccess(var_name) => context
            .get(&var_name)
            .ok_or_else(|| format!("Name '{}' is not defined", var_name))?,
    })
}

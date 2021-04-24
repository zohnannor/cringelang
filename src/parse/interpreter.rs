use std::{collections::HashMap, rc::Rc};

use super::parser::ASTNode;
use super::tokens::{Operator, Parenthesis, Token};
use crate::stdlib::{number::Number, object::Object};

#[derive(Default, Debug)]
pub struct Context {
    symbols: HashMap<String, Rc<dyn Object>>,
    parent: Option<Box<Context>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get(&self, var_name: &String) -> Option<Rc<dyn Object>> {
        Some(match self.symbols.get(var_name) {
            Some(val) => Rc::clone(val),
            None => self.parent.as_ref()?.get(var_name)?,
        })
    }

    fn set(&mut self, var_name: String, value: &Rc<dyn Object>) {
        self.symbols.insert(var_name, Rc::clone(value));
    }
}

pub fn visit(node: ASTNode, context: &mut Context) -> Result<Rc<dyn Object>, String> {
    Ok(match node {
        ASTNode::Number(n) => Rc::new(n),
        ASTNode::Bool(b) => Rc::new(b),
        ASTNode::UnOp(op, node) => match op {
            Operator::Plus => visit(*node, context)?,
            Operator::Minus => visit(*node, context)?.mul(Rc::new(Number::Int(-1)))?,
            _ => todo!(),
        },
        ASTNode::BinOp(left_node, op, right_node) => match op {
            Operator::Plus => visit(*left_node, context)?.add(visit(*right_node, context)?)?,
            Operator::Minus => visit(*left_node, context)?.sub(visit(*right_node, context)?)?,
            Operator::Star => visit(*left_node, context)?.mul(visit(*right_node, context)?)?,
            Operator::Slash => visit(*left_node, context)?.div(visit(*right_node, context)?)?,
            Operator::StarStar => visit(*left_node, context)?.pow(visit(*right_node, context)?)?,
            Operator::Percent => visit(*left_node, context)?.r#mod(visit(*right_node, context)?)?,
            _ => todo!(),
        },
        ASTNode::VarCreate(var_name, node) => {
            let val = visit(*node, context)?;
            context.set(var_name, &val);
            val
        }
        ASTNode::VarAssign(var_name, node) => {
            if context.symbols.contains_key(&var_name) {
                let val = visit(*node, context)?;
                context.set(var_name, &val);
                val
            } else {
                return Err(format!("Name '{}' is not defined", var_name));
            }
        }
        ASTNode::VarAccess(var_name) => context
            .get(&var_name)
            .ok_or_else(|| format!("Name '{}' is not defined", var_name))?,
    })
}

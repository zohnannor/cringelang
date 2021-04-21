use std::collections::HashMap;

use super::lexer::{Operator, Parenthesis};
use super::parser::ASTNode;
use crate::stdlib::number::Number;

#[derive(Default, Debug)]
pub struct Context {
    symbols: HashMap<String, Number>,
    parent: Option<Box<Context>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get(&self, var_name: &String) -> Option<Number> {
        Some(match self.symbols.get(var_name) {
            Some(val) => *val,
            None => return self.parent.as_ref()?.get(var_name),
        })
    }

    fn set(&mut self, var_name: String, value: Number) {
        self.symbols.insert(var_name, value);
    }
}

pub fn visit(node: ASTNode, context: &mut Context) -> Result<Number, String> {
    Ok(match node {
        ASTNode::Number(n) => n,
        ASTNode::UnOp(op, node) => match op {
            Operator::Plus => visit(*node, context)?,
            Operator::Minus => visit(*node, context)?.mul(Number::Int(-1)),
            _ => todo!(),
        },
        ASTNode::BinOp(left_node, op, right_node) => match op {
            Operator::Plus => visit(*left_node, context)?.add(visit(*right_node, context)?),
            Operator::Minus => visit(*left_node, context)?.sub(visit(*right_node, context)?),
            Operator::Star => visit(*left_node, context)?.mul(visit(*right_node, context)?),
            Operator::Slash => visit(*left_node, context)?.div(visit(*right_node, context)?),
            Operator::StarStar => visit(*left_node, context)?.pow(visit(*right_node, context)?)?,
            Operator::Percent => visit(*left_node, context)?.r#mod(visit(*right_node, context)?)?,
            _ => todo!(),
        },
        ASTNode::VarCreate(var_name, node) => {
            let val = visit(*node, context)?;
            context.set(var_name, val);
            val
        }
        ASTNode::VarAssign(var_name, node) => {
            if context.symbols.contains_key(&var_name) {
                let val = visit(*node, context)?;
                context.set(var_name, val);
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

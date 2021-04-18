use lexer::Operator;

use crate::{lexer, number::Number, parser::ASTNode};

#[derive(Debug)]
pub struct RTResult;

pub fn visit(node: ASTNode) -> Result<Number, String> {
    Ok(match node {
        ASTNode::Number(n) => n,
        ASTNode::Ident(_) => unreachable!(),
        ASTNode::UnOp(op, node) => match op {
            Operator::Plus => visit(*node)?,
            Operator::Minus => visit(*node)?.mul(Number::Int(-1)),
            _ => unreachable!(),
        },
        ASTNode::BinOp(left_node, op, right_node) => match op {
            Operator::Plus => visit(*left_node)?.add(visit(*right_node)?),
            Operator::Minus => visit(*left_node)?.sub(visit(*right_node)?),
            Operator::Star => visit(*left_node)?.mul(visit(*right_node)?),
            Operator::Slash => visit(*left_node)?.div(visit(*right_node)?),
            Operator::StarStar => visit(*left_node)?.pow(visit(*right_node)?)?,
            Operator::Percent => visit(*left_node)?.r#mod(visit(*right_node)?)?,
            _ => unreachable!(),
        },
    })
}

use crate::public::compile_time::ast::types::{AssignmentNode, ExpressionNode};
use crate::public::error::assignment_error;
use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::compiler::tokenizer::token::TokenVec;

use super::super::expression;

pub fn resolve(
    tokens: &mut TokenVec,
    equal_symbol: Symbols,
    left_hand_node: ASTNode,
) -> Result<AssignmentNode, ()> {
    // assignment
    // `symbol` may be: += | -= | *= | /= | ^=

    let mut right_hand_node =
        expression::resolve(tokens)?;

    if right_hand_node.elements.len() == 0 {
        // example:
        // var =
        return Err(assignment_error("missing right-hand value.")?)
    }

    if equal_symbol != Symbols::Equal {
        let origin_node = ASTNode::Expression(
            right_hand_node.into()
        );
        let separated = equal_symbol.separate();
        let variable_node = left_hand_node.clone();
        let symbol_node = ASTNode::SymbolLiteral(separated);

        right_hand_node = ExpressionNode {
            elements: vec![variable_node, origin_node, symbol_node]
        };
    }

    let current_node = AssignmentNode {
        left_hand_node,
        right_hand_node,
    };

    Ok(current_node)
}
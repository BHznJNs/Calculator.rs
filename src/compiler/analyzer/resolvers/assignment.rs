use std::rc::Rc;

use crate::public::value::symbols::Symbols;
use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::compiler::tokenizer::token::TokenVec;

use super::expression;


pub fn resolve(
    tokens: &mut TokenVec,
    equal_symbol: Symbols,
    left_hand_node: ASTNode,
) -> Result<ASTNode, ()> {
    // assignment
    // `symbol` may be: += | -= | *= | /= | ^=

    let mut right_hand_nodes =
        expression::resolve(tokens, false)?;

    if right_hand_nodes.len() == 0 {
        // example:
        // var =
        println!("Invalid assignment.");
        return Err(())
    }

    let right_hand_expression =
    if let  ASTNodeTypes::LazyExpression   | // lazy-expression
            ASTNodeTypes::ArrayLiteral     | // array assignment
            ASTNodeTypes::ClassDefinition  | // class definition
            ASTNodeTypes::Instantiation(_) | // class instantiation
            ASTNodeTypes::FunctionDefinition(_) = // function
            right_hand_nodes[0].type__ {
        right_hand_nodes.remove(0)
    } else {
        let original = ASTNode {
            type__: ASTNodeTypes::Expression,
            params: Some(right_hand_nodes)
        };
        // variable assignment
        if equal_symbol == Symbols::Equal {
            // directly assignment
            original
        } else {
            // resolve:   += | -= | *= | /= | ^=
            // separated: +  | -  | *  | /  | ^

            // example:
            // input : var += 1 
            // output: var = var 1 +
            //               ^^^^^^^ using postfix-expression
            let separated = equal_symbol.separate();
            let variable_node = left_hand_node.clone();
            let symbol_node = ASTNode {
                type__: ASTNodeTypes::SymbolLiteral(separated),
                params: None,
            };
            ASTNode {
                type__: ASTNodeTypes::Expression,
                // postfix-expression
                params: Some(vec![variable_node, original, symbol_node])
            }
        }
    };
    let current_node = ASTNode {
        type__: ASTNodeTypes::Assignment(Rc::new(left_hand_node)),
        params: Some(vec![right_hand_expression]),
    };

    Ok(current_node)
}
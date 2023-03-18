use crate::public::symbols::Symbols;
use crate::public::ast::{ASTNode, ASTNodeTypes};

fn get_priority(symbol_node: &ASTNode) -> Result<i8, ()> {
    if let ASTNodeTypes::SymbolLiteral(symbol) = symbol_node.type__ {
        match symbol {
            Symbols::Plus       => Ok(1),
            Symbols::Minus      => Ok(1),
            Symbols::Multiply   => Ok(2),
            Symbols::Divide     => Ok(2),
            Symbols::Power      => Ok(3),
            _ => {
                println!("Invalid symbol: '{}'.", symbol);
                return Err(())
            }
        }
    } else {
        println!("Analyzer error.");
        return Err(())
    }
}

pub fn compare(
    symbol_node1: &ASTNode,
    symbol_node2: &ASTNode,
) -> Result<i8, ()> {
    let priority1 = get_priority(symbol_node1)?;
    let priority2 = get_priority(symbol_node2)?;

    if priority1 > priority2 {
        Ok(1)
    } else if priority1 == priority2 {
        Ok(0)
    } else {
        Ok(-1)
    }
}
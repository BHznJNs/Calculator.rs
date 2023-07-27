use crate::public::{
    compile_time::ast::ast_enum::ASTNode,
    error::{internal_error, InternalComponent},
};

const PRIORITY: [i8; 14] = [
    2, // Symbols::Plus
    2, // Symbols::Minus
    3, // Symbols::Multiply
    3, // Symbols::Divide
    4, // Symbols::Power
    5, // Symbols::Not
    1, // Symbols::LessThan
    1, // Symbols::MoreThan
    1, // Symbols::LessThanEqual
    1, // Symbols::MoreThanEqual
    1, // Symbols::CompareEqual
    1, // Symbols::NotEqual
    0, // Symbols::AndSign
    0, // Symbols::OrSign
];

fn get_priority(symbol_node: &ASTNode) -> Result<i8, ()> {
    if let ASTNode::SymbolLiteral(symbol) = symbol_node {
        let symbol_index = *symbol as usize;
        if symbol_index >= PRIORITY.len() {
            let msg = format!("invalid symbol `{}`", symbol);
            return Err(internal_error(InternalComponent::Analyzer, &msg)?);
        }
        Ok(PRIORITY[symbol_index])
    } else {
        let msg = format!("invalid ASTNode for `get_priority`: {}", symbol_node);
        return Err(internal_error(InternalComponent::Analyzer, &msg)?);
    }
}

pub fn compare(symbol_node1: &ASTNode, symbol_node2: &ASTNode) -> Result<i8, ()> {
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

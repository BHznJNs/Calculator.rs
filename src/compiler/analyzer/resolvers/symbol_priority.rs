use crate::public::compile_time::ast::ast_enum::ASTNode;

const PRIORITY: [i8; 12] = [
    1, // Symbols::Plus
    1, // Symbols::Minus
    2, // Symbols::Multiply
    2, // Symbols::Divide
    3, // Symbols::Power
    5, // Symbols::Not
    0, // Symbols::LessThan
    0, // Symbols::MoreThan
    0, // Symbols::LessThanEqual
    0, // Symbols::MoreThanEqual
    0, // Symbols::CompareEqual
    0, // Symbols::NotEqual
];

fn get_priority(symbol_node: &ASTNode) -> Result<i8, ()> {
    if let ASTNode::SymbolLiteral(symbol) = symbol_node {
        let symbol_index = *symbol as usize;
        if symbol_index >= PRIORITY.len() {
            println!("AnalyzerError: invalid symbol: `{}`.", symbol);
            return Err(());
        }
        Ok(PRIORITY[symbol_index])
    } else {
        println!("Analyzer error from 'get_priority'.");
        return Err(());
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

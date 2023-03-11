use std::fmt;
use super::number::Number;
use super::symbols::Symbols;

#[derive(PartialEq, Clone)]
pub enum ASTNodeTypes {
    Root,

    Variable(String),
    Assignment(String),
    NumberLiteral(Number),
    SymbolLiteral(Symbols),
    Expression,
    GotoStatement,
    InvokeExpression(String),
}

impl fmt::Display for ASTNodeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("ASTNode: ");
        match self {
            ASTNodeTypes::Root => write!(f, "AST Root"),
            ASTNodeTypes::Variable(name) => write!(f, "type: Variable, name {}", name),
            ASTNodeTypes::Assignment(name) => write!(f, "type: Assignment, name {}", name),
            ASTNodeTypes::NumberLiteral(num) => write!(f, "type: NumberLiteral, value: {}", num),
            ASTNodeTypes::SymbolLiteral(symbol) => write!(f, "type: SymbolLiteral, value: {}", symbol),
            ASTNodeTypes::Expression => write!(f, "type: Expression"),
            ASTNodeTypes::GotoStatement => write!(f, "type: GotoStatement"),
            ASTNodeTypes::InvokeExpression(name) => write!(f, "type: InvokeExpression, name: {}", name),
        }
    }
}

// --- --- --- --- --- --- --- ---

#[derive(Clone)]
pub struct ASTNode {
    pub type__: ASTNodeTypes,
    pub params: Option<ASTNodeVec>,
}
pub type ASTNodeVec = Vec<ASTNode>;

impl fmt::Display for ASTNode {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{}", self.type__);
        match &self.params {
            Some(params) => {
                print!("params: {{\n");
                for node in params {
                    println!("{}", node);
                }
                print!("}}");
                Ok(())
            },
            None => Ok(()),
        }
    }
}
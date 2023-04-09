use std::fmt;
use std::rc::Rc;

use crate::public::value::function::Param;
use crate::public::value::number::Number;
use crate::public::value::symbols::Symbols;

use super::keywords::Keywords;

#[derive(PartialEq, Clone)]
pub enum ASTNodeTypes {
    Root,
    Comment,

    Variable(String),
    Assignment(Rc<ASTNode>),
    NumberLiteral(Number),
    StringLiteral(String),
    SymbolLiteral(Symbols),
    ArrayLiteral,
    ArrayElementReading(String),
    Expression,
    LazyExpression,

    Invocation(String),
    Statement(Keywords),

    FunctionDefinition(Vec<Param>),
    ClassDefinition,
    Instantiation(String),
    ObjectReading(String), // (obj_name, property_name)
}

// display for debug
impl fmt::Display for ASTNodeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNodeTypes::Root => write!(f, "AST Root"),
            ASTNodeTypes::Comment => write!(f, "type: Comment"),

            ASTNodeTypes::Variable(name) => write!(f, "type: Variable, name: {}", name),
            ASTNodeTypes::Assignment(name) => write!(f, "type: Assignment, name: {}", name),
            ASTNodeTypes::NumberLiteral(num) => write!(f, "type: NumberLiteral, value: {}", num),
            ASTNodeTypes::StringLiteral(str) => write!(f, "type: StringLiteral, value: {}", str),
            ASTNodeTypes::SymbolLiteral(symbol) => write!(f, "type: SymbolLiteral, value: {}", symbol),
            ASTNodeTypes::ArrayLiteral => write!(f, "type: ArrayLiteral"),
            ASTNodeTypes::ArrayElementReading(array_name) => write!(f, "type: ArrayElementReading, name: {}", array_name),

            ASTNodeTypes::Expression => write!(f, "type: Expression"),
            ASTNodeTypes::LazyExpression => write!(f, "type: LazyExpression"),
            ASTNodeTypes::FunctionDefinition(_) => write!(f, "type: UserDefinedFunction"),
            ASTNodeTypes::ClassDefinition => write!(f, "type: UserDefinedClass"),
            ASTNodeTypes::Invocation(name) => write!(f, "type: Invocation, name: {}", name),
            ASTNodeTypes::Statement(keyword) => write!(f, "type: Statement, keyword: {}", keyword),
            ASTNodeTypes::Instantiation(class_name) => write!(f, "type: Instantiation, name: {}", class_name),
            ASTNodeTypes::ObjectReading(obj_name) => write!(f, "type: ObjectReading, obj: {:?}", obj_name),
        }
    }
}

// --- --- --- --- --- --- --- ---

#[derive(PartialEq, Clone)]
pub struct ASTNode {
    pub type__: ASTNodeTypes,
    pub params: Option<ASTNodeVec>,
}
pub type ASTNodeVec = Vec<ASTNode>;

impl fmt::Display for ASTNode {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("ASTNode: {}", self.type__);
        match &self.params {
            Some(params) => {
                // if sub-params,
                // recursively show sub-params
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
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
    ArrayElementReading(Rc<ASTNode>),
    Expression,
    LazyExpression,

    Invocation(Rc<ASTNode>),
    Statement(Keywords),

    FunctionDefinition(Vec<Param>),
    ClassDefinition,
    Instantiation(String),
    ObjectReading(Rc<ASTNode>), // (obj_name, property_name)
}

// display for debug
impl fmt::Display for ASTNodeTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNodeTypes::Root => write!(f, "AST Root"),
            ASTNodeTypes::Comment => write!(f, "type: Comment"),

            ASTNodeTypes::Variable(name) => write!(f, "type: Variable, name: {}", name),
            ASTNodeTypes::Assignment(left_hand) => write!(f, "type: Assignment, left_hand:\n  {}", left_hand),
            ASTNodeTypes::NumberLiteral(num) => write!(f, "type: NumberLiteral, value: {}", num),
            ASTNodeTypes::StringLiteral(str) => write!(f, "type: StringLiteral, value: {}", str),
            ASTNodeTypes::SymbolLiteral(symbol) => write!(f, "type: SymbolLiteral, value: {}", symbol),
            ASTNodeTypes::ArrayLiteral => write!(f, "type: ArrayLiteral"),
            ASTNodeTypes::ArrayElementReading(arr_node) => write!(f, "type: ArrayElementReading, arr_node:\n  {}", arr_node),

            ASTNodeTypes::Expression => write!(f, "type: Expression"),
            ASTNodeTypes::LazyExpression => write!(f, "type: LazyExpression"),
            ASTNodeTypes::FunctionDefinition(_) => write!(f, "type: UserDefinedFunction"),
            ASTNodeTypes::ClassDefinition => write!(f, "type: UserDefinedClass"),
            ASTNodeTypes::Invocation(caller) => write!(f, "type: Invocation, caller:\n  {}", caller),
            ASTNodeTypes::Statement(keyword) => write!(f, "type: Statement, keyword: {}", keyword),
            ASTNodeTypes::Instantiation(class_name) => write!(f, "type: Instantiation, name: {}", class_name),
            ASTNodeTypes::ObjectReading(obj_node) => write!(f, "type: ObjectReading, obj:\n  {}", obj_node),
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
                print!("params: {{\n  ");
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
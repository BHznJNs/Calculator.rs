use core::fmt;
use std::rc::Rc;

use crate::public::value::{number::Number, symbols::Symbols};

use super::types::{
    ArrayElementReadingNode, ArrayLiteralNode, AssignmentNode, ClassDefinitionNode, ExpressionNode,
    FunctionDefinitionNode, InstantiationNode, InvocationNode, LazyExpressionNode,
    ObjectReadingNode, StatementNode, VariableNode, ImportNode,
};

pub struct RootNode {
    pub sub_node: ASTNode,
}

#[derive(PartialEq, Clone)]
pub enum ASTNode {
    Comment,

    NumberLiteral(Number),
    StringLiteral(String),
    SymbolLiteral(Symbols),

    Variable(Box<VariableNode>),
    Assignment(Box<AssignmentNode>),
    ArrayLiteral(Box<ArrayLiteralNode>),
    ArrayElementReading(Box<ArrayElementReadingNode>),
    Expression(Box<ExpressionNode>),
    LazyExpression(Box<LazyExpressionNode>),

    Invocation(Box<InvocationNode>),
    Statement(Box<StatementNode>),

    ImportStatement(Box<ImportNode>),
    FunctionDefinition(Box<FunctionDefinitionNode>),
    ClassDefinition(Box<ClassDefinitionNode>),
    Instantiation(Box<InstantiationNode>),
    ObjectReading(Box<ObjectReadingNode>),
}

pub type ASTVec = Vec<ASTNode>;

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
            ASTNode::Comment => "Comment",
            ASTNode::NumberLiteral(_) => "NumberLiteral",
            ASTNode::StringLiteral(_) => "StringLiteral",
            ASTNode::SymbolLiteral(_) => "SymbolLiteral",
            ASTNode::Variable(_) => "Variable",
            ASTNode::Assignment(_) => "Assignment",
            ASTNode::ArrayLiteral(_) => "ArrayLiteral",
            ASTNode::ArrayElementReading(_) => "ArrayElementReading",
            ASTNode::Expression(_) => "Expression",
            ASTNode::LazyExpression(_) => "LazyExpression",
            ASTNode::Invocation(_) => "Invocation",
            ASTNode::Statement(_) => "Statement",
            ASTNode::ImportStatement(_) => "ImportStatement",
            ASTNode::FunctionDefinition(_) => "FunctionDefinition",
            ASTNode::ClassDefinition(_) => "ClassDefinition",
            ASTNode::Instantiation(_) => "Instantiation",
            ASTNode::ObjectReading(_) => "ObjectReading",
        };
        write!(f, "(ASTNode: {})", content)
    }
}

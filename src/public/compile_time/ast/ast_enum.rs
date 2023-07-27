use core::fmt;

use crate::public::value::{number::Number, symbols::Symbols};

use super::types::{
    ArrayElementReadingNode, ArrayLiteralNode, AssignmentNode, ClassDefinitionNode, ExpressionNode,
    FunctionDefinitionNode, ImportNode, InstantiationNode, InvocationNode, LazyExpressionNode,
    ObjectReadingNode, StatementNode, VariableNode,
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
            Self::Comment => "Comment",
            Self::NumberLiteral(_) => "NumberLiteral",
            Self::StringLiteral(_) => "StringLiteral",
            Self::SymbolLiteral(_) => "SymbolLiteral",
            Self::Variable(_) => "Variable",
            Self::Assignment(_) => "Assignment",
            Self::ArrayLiteral(_) => "ArrayLiteral",
            Self::ArrayElementReading(_) => "ArrayElementReading",
            Self::Expression(_) => "Expression",
            Self::LazyExpression(_) => "LazyExpression",
            Self::Invocation(_) => "Invocation",
            Self::Statement(_) => "Statement",
            Self::ImportStatement(_) => "ImportStatement",
            Self::FunctionDefinition(_) => "FunctionDefinition",
            Self::ClassDefinition(_) => "ClassDefinition",
            Self::Instantiation(_) => "Instantiation",
            Self::ObjectReading(_) => "ObjectReading",
        };
        write!(f, "(ASTNode: {})", content)
    }
}

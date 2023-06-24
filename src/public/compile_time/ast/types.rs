use std::rc::Rc;

use crate::public::value::function::UserDefinedFnParam;
use crate::public::value::oop::class::Property;

use super::ast_enum::{ASTNode, ASTVec};

#[derive(PartialEq, Clone)]
pub struct VariableNode {
    pub name: String,
}
#[derive(PartialEq, Clone)]
pub struct AssignmentNode {
    pub left_hand_node: ASTNode,
    pub right_hand_node: ExpressionNode,
}
#[derive(PartialEq, Clone)]
pub struct ArrayLiteralNode {
    pub elements: Vec<ExpressionNode>,
}
#[derive(PartialEq, Clone)]
pub struct ArrayElementReadingNode {
    pub array_node: ASTNode,
    pub index_node: ExpressionNode,
}
#[derive(PartialEq, Clone)]
pub struct ExpressionNode {
    pub elements: ASTVec,
}
#[derive(PartialEq, Clone)]
pub struct LazyExpressionNode {
    pub sub_sequence: ASTNode,
}
#[derive(PartialEq, Clone)]
pub struct InvocationNode {
    pub caller: ASTNode,
    pub params: Vec<ExpressionNode>,
}
#[derive(PartialEq, Clone)]
pub struct ObjectReadingNode {
    pub obj_node: ASTNode,
    pub property: String,
}

#[derive(PartialEq, Clone)]
pub struct FunctionDefinitionNode {
    pub params: Vec<UserDefinedFnParam>,
    pub name: Option<String>,
    pub body: ASTVec,
}
#[derive(PartialEq, Clone)]
pub struct ClassDefinitionNode {
    pub properties: Vec<Property>,
    pub method_nodes: Vec<Rc<FunctionDefinitionNode>>,
}
#[derive(PartialEq, Clone)]
pub struct InstantiationNode {
    pub class: String,
    pub params: ArrayLiteralNode,
}

// --- --- --- ---|
// Statement Node |
// --- --- --- ---|
#[derive(PartialEq, Clone)]
pub enum StatementNode {
    Output(ExpressionNode),
    ForLoop(ForStatement),
    Condition(IfStatement),
    Import(ImportNode),

    Continue,
    Break(ExpressionNode),
}
#[derive(PartialEq, Clone)]
pub struct ForStatement {
    pub loop_count: ExpressionNode,
    pub body: ASTVec,
}
#[derive(PartialEq, Clone)]
pub struct IfStatement {
    pub condition: ExpressionNode,
    pub body: ASTVec,
}

#[derive(PartialEq, Clone)]
pub enum ModuleType {
    BuildIn,
    UserDefined,
}
#[derive(PartialEq, Clone)]
pub struct ImportNode {
    pub type__: ModuleType,
    pub target: String,
}

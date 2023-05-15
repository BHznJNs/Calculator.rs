use crate::public::value::function::Param;
use crate::public::compile_time::keywords::Keywords;

use super::ast_enum::{ASTNode, ASTVec};

#[derive(PartialEq, Clone)]
pub struct VariableNode {
    pub name: String
}
#[derive(PartialEq, Clone)]
pub struct AssignmentNode {
    pub left_hand_node : ASTNode,
    pub right_hand_node: ExpressionNode,
}
#[derive(PartialEq, Clone)]
pub struct ArrayLiteralNode {
    pub elements: Vec<ExpressionNode>
}
#[derive(PartialEq, Clone)]
pub struct ArrayElementReadingNode {
    pub array_node: ASTNode,
    pub index_node: ExpressionNode,
}
#[derive(PartialEq, Clone)]
pub struct ExpressionNode {
    pub elements: ASTVec
}
#[derive(PartialEq, Clone)]
pub struct LazyExpressionNode {
    pub sub_sequence: ASTNode
}
#[derive(PartialEq, Clone)]
pub struct InvocationNode {
    pub caller: ASTNode,
    pub params: Vec<ExpressionNode>,
}
#[derive(PartialEq, Clone)]
pub struct StatementNode {
    pub keyword: Keywords,
    pub condition: Option<ExpressionNode>,
    pub body: ASTVec,
}
#[derive(PartialEq, Clone)]
pub struct FunctionDefinitionNode {
    pub params: Vec<Param>,
    pub name: Option<String>,
    pub body: ASTVec,
}
#[derive(PartialEq, Clone)]
pub struct ClassDefinitionNode {
    pub params: ASTVec
}
#[derive(PartialEq, Clone)]
pub struct InstantiationNode {
    pub class: String,
    pub params: ArrayLiteralNode,
}
#[derive(PartialEq, Clone)]
pub struct ObjectReadingNode {
    pub obj_node: ASTNode,
    pub property: String,
}
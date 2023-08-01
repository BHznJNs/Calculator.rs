use std::rc::Rc;

use crate::public::value::function::UserDefinedFnParam;
use crate::public::value::oop::class::Property;

use super::ast_enum::{ASTNode, ASTVec};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct VariableNode {
    pub name: String,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct AssignmentNode {
    pub left_hand_node: ASTNode,
    pub right_hand_node: ExpressionNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ArrayLiteralNode {
    pub elements: Vec<ExpressionNode>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct MapLiteralNode {
    pub keys: Vec<String>,
    pub values: Vec<ExpressionNode>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ElementReadingNode {
    pub target_node: ASTNode,
    pub index_node: ExpressionNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ExpressionNode {
    pub elements: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct LazyExpressionNode {
    pub sub_sequence: ASTNode,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct InvocationNode {
    pub caller: ASTNode,
    pub params: Vec<ExpressionNode>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ObjectReadingNode {
    pub obj_node: ASTNode,
    pub property: String,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct FunctionDefinitionNode {
    pub params: Vec<UserDefinedFnParam>,
    pub name: Option<String>,
    pub body: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ClassDefinitionNode {
    pub properties: Vec<Property>,
    pub method_nodes: Vec<Rc<FunctionDefinitionNode>>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct InstantiationNode {
    pub class: String,
    pub params: ArrayLiteralNode,
}

// --- --- --- ---|
// Statement Node |
// --- --- --- ---|
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum StatementNode {
    Output(ExpressionNode),
    ForLoop(ForStatement),
    Condition(IfStatement),
    Import(ImportNode),
    GlobalAssignment(AssignmentNode),

    Continue,
    Break(ExpressionNode),
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ForStatement {
    pub loop_count: ExpressionNode,
    pub body: ASTVec,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct IfStatement {
    pub condition: ExpressionNode,
    pub body: ASTVec,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum ModuleType {
    BuildIn,
    UserDefined,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct ImportNode {
    pub type__: ModuleType,
    pub target: String,
}

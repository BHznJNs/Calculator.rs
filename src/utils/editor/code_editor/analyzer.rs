use crate::{
    computer,
    exec::script::script_analyze,
    public::{
        compile_time::ast::{
            ast_enum::{ASTNode, RootNode},
            types::{ExpressionNode, FunctionDefinitionNode},
        },
        run_time::scope::Scope,
        value::Value,
    },
};

fn complete_brace(context_lines: Vec<String>, current_indent: usize) -> Vec<String> {
    if context_lines.is_empty() {
        return vec![];
    }

    let mut result_lines = context_lines;
    for indent in (1..current_indent).rev() {
        let indent_str = "  ".repeat(indent);
        result_lines.push(indent_str + "}");
    }
    return result_lines;
}

fn get_func_def_node(expression_node: &ExpressionNode) -> Option<&FunctionDefinitionNode> {
    let ASTNode::Assignment(assignment_node) = &expression_node.elements[0] else {
        return None;
    };
    let ASTNode::FunctionDefinition(func_def_node) = &assignment_node.right_hand_node.elements[0]
    else {
        return None;
    };
    return Some(func_def_node);
}

fn execute_global(node_list: Vec<RootNode>, scope: &mut Scope) {
    // for global scope code, execute line by line.
    for node in node_list {
        let _ = computer::compute(node, scope);
    }
}

fn execute_local(local_node: Option<&FunctionDefinitionNode>, scope: &mut Scope) {
    // for local scope, first to get the nearest function definition node
    let mut nearest_func_def_node = local_node;
    while let Some(func_def_node) = nearest_func_def_node {
        let last_node = func_def_node
            .body
            .iter()
            .rev()
            .find(|node| **node != ASTNode::Comment);
        let Some(ASTNode::Expression(expression_node)) = last_node else {
            break;
        };
        let Some(sub_func_def_node) = get_func_def_node(expression_node) else {
            break;
        };
        nearest_func_def_node = Some(sub_func_def_node);
    }

    let Some(func_def_node) = nearest_func_def_node else {
        return;
    };
    // assign function params
    for param in &func_def_node.params {
        scope.assign(param.identi.to_string(), Value::null());
    }
    // execute local codes
    for node in &func_def_node.body {
        let root_node = RootNode {
            sub_node: node.clone(),
        };
        let _ = computer::compute(root_node, scope);
    }
}

pub fn analyze(context_lines: Vec<String>, current_indent: usize) -> Scope {
    // default scope
    let mut scope = Scope::new();
    scope.init_completer(true);

    let context_lines = complete_brace(context_lines, current_indent);
    let mut ast_list = script_analyze(context_lines);
    if ast_list.is_empty() {
        return scope;
    }
    let last_node = ast_list.pop().unwrap();
    let is_last_node_func_def = matches!(
        &last_node.sub_node,
        ASTNode::Expression(expression_node)
            if get_func_def_node(expression_node).is_some()
    );
    let (global_node_list, local_node) = if is_last_node_func_def {
        let ASTNode::Expression(expression_node) = &last_node.sub_node else {
            unreachable!()
        };
        let func_def_node = get_func_def_node(expression_node).unwrap();
        (ast_list, Some(func_def_node))
    } else {
        ast_list.push(last_node);
        (ast_list, None)
    };

    execute_global(global_node_list, &mut scope);
    execute_local(local_node, &mut scope);

    return scope;
}

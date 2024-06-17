use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::{
    exec::script::{read_file, script_analyze},
    public::{
        compile_time::{
            ast::{
                ast_enum::{ASTNode, RootNode},
                types::{
                    AssignmentNode, ExpressionNode, FunctionDefinitionNode, StatementNode
                },
            },
            keywords::Keyword,
        }, run_time::{
            constants::CONSTANT_IDENTIFIERS,
            scope::{Scope, STD_MODULE_COUNT}
        },
        std::StdModule,
        value::{
            oop::{
                class::{Class, Property},
                object::Object,
            },
            ValueType,
        },
    },
    utils::{completer::Completer, log},
};

#[derive(Clone)]
struct ModuleCache {
    pub var_list: HashSet<String>,
    pub type_list: HashSet<String>,
    pub obj_class_map: HashMap<String, Rc<Class>>,
    pub class_name_map: HashMap<String, Rc<Class>>,
}

impl ModuleCache {
    pub fn new() -> Self {
        Self {
            var_list: HashSet::new(),
            type_list: HashSet::new(),
            obj_class_map: HashMap::new(),
            class_name_map: HashMap::new(),
        }
    }
}

impl From<EditorScope> for ModuleCache {
    fn from(value: EditorScope) -> Self {
        let mut var_list = HashSet::new();
        let mut type_list = HashSet::new();
        var_list.extend(value.var_completer.elements().into_iter());
        type_list.extend(value.type_completer.elements().into_iter());

        return Self {
            var_list,
            type_list,
            obj_class_map: value.obj_class_map,
            class_name_map: value.class_name_map,
        };
    }
}

// --- --- --- --- --- ---

#[derive(Clone)]
pub struct EditorScope {
    user_module_imported: HashSet<String>,
    std_module_imported: [bool; STD_MODULE_COUNT],

    var_completer: Completer,
    type_completer: Completer,
    obj_class_map: HashMap<String, Rc<Class>>,
    class_name_map: HashMap<String, Rc<Class>>,
}

impl EditorScope {
    pub fn new() -> Self {
        let mut var_completer = Completer::new();
        var_completer.extend(CONSTANT_IDENTIFIERS);
        var_completer.extend(Keyword::keyword_list());

        let mut type_completer = Completer::new();
        type_completer.extend(ValueType::build_in_type_list());

        return Self {
            user_module_imported: HashSet::new(),
            std_module_imported: [false; STD_MODULE_COUNT],

            var_completer,
            type_completer,
            obj_class_map: HashMap::new(),
            class_name_map: HashMap::new(),
        };
    }

    fn insert_var(&mut self, word: &str) {
        self.var_completer.insert(word);
    }
    fn insert_obj(&mut self, obj_name: &str, class_name: &str) {
        if let Some(class) = self.class_name_map.get(class_name) {
            self.obj_class_map.insert(obj_name.to_string(), class.clone());
        }
    }
    fn insert_class(&mut self, class_name: &str, class_inst: Class) {
        self.type_completer.insert(class_name);
        self.class_name_map.insert(class_name.to_string(), class_inst.into());
    }

    fn extend(&mut self, cached: &ModuleCache) {
        self.var_completer.extend(cached.var_list.iter());
        self.type_completer.extend(cached.type_list.iter());
        self.obj_class_map.extend(cached.obj_class_map.clone().into_iter());
        self.class_name_map.extend(cached.class_name_map.clone().into_iter());
    }

    fn merge(&mut self, other: Self) {
        for (index, is_imported) in other.std_module_imported.into_iter().enumerate() {
            if is_imported {
                self.std_module_imported[index] = is_imported;
            }
        }
        for user_module in other.user_module_imported.into_iter() {
            self.user_module_imported.insert(user_module);
        }

        self.var_completer.extend(other.var_completer.elements().iter());
        self.type_completer.extend(other.type_completer.elements().iter());
        self.obj_class_map.extend(other.obj_class_map.into_iter());
        self.class_name_map.extend(other.class_name_map.into_iter());
    }

    fn import_std(&mut self, module_name: &str) {
        const ARRAY_REPEAT_VALUE: Option<ModuleCache> = None;
        static mut STD_MODULE_CACHE: [Option<ModuleCache>; STD_MODULE_COUNT] = [ARRAY_REPEAT_VALUE; STD_MODULE_COUNT];

        let Some(target_module) = Scope::std_module_map().get(module_name) else {
            return;
        };
        if self.std_module_imported[*target_module as usize] {
            return;
        }

        if let Some(cached) = unsafe { &STD_MODULE_CACHE[*target_module as usize] } {
            // directly use cache
            self.extend(cached);
            return;
        }

        self.std_module_imported[*target_module as usize] = true;
        let mut scope = ModuleCache::new();
        match target_module {
            StdModule::Basic | StdModule::BitOps => {
                let func_list = target_module.get_fn_list();
                for (func_name, _) in func_list {
                    scope.var_list.insert(func_name);
                }
            }
            StdModule::FileSystem | StdModule::Math => {
                let Object::BuildIn(module_obj) = target_module.get_obj_entry() else {
                    unreachable!()
                };
                scope.var_list.insert(module_name.to_string());
                let methods = module_obj
                    .get_method_list()
                    .into_iter()
                    .map(|method| Property(ValueType::Function, method.to_string()))
                    .collect();
                let class = Class::new(methods, vec![]);
                scope.obj_class_map.insert(module_name.to_string(), class.into());
            }
            StdModule::String | StdModule::Array | StdModule::Map => {
                let module_cls = target_module.get_cls_entry();
                scope.class_name_map.insert(module_name.to_string(), module_cls);
            }
        }
        self.extend(&scope);
        unsafe { STD_MODULE_CACHE[*target_module as usize] = Some(scope) }
    }

    fn import_from_path(&mut self, module_path: &str) {
        static mut USER_MODULE_CACHE: Option<HashMap<String, ModuleCache>> = None;
        let cache = unsafe {
            if let Some(cache) = USER_MODULE_CACHE.as_ref() {
                cache
            } else {
                USER_MODULE_CACHE = Some(HashMap::new());
                USER_MODULE_CACHE.as_ref().unwrap()
            }
        };

        if self.user_module_imported.contains(module_path) {
            return;
        }
        self.user_module_imported.insert(module_path.to_string());
        if let Some(cached) = cache.get(module_path) {
            self.extend(cached);
            return;
        }

        let mut module_scope = EditorScope::new();
        let Ok(file_content) = read_file(module_path) else {
            return;
        };
        let lines = file_content
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let ast_list = script_analyze(lines)
            .into_iter()
            .map(|root_node| root_node.sub_node)
            .collect();
        analyze_scope(ast_list, &mut module_scope);
        self.merge(module_scope.clone());
        unsafe {
            USER_MODULE_CACHE
                .as_mut()
                .unwrap()
                .insert(
                    module_path.to_string(),
                    ModuleCache::from(module_scope),
                );
        }
    }

    pub fn get_class_by_obj(&self, obj_name: &str) -> Option<&Rc<Class>> {
        self.obj_class_map.get(obj_name)
    }
    pub fn get_class_by_name(&self, class_name: &str) -> Option<&Rc<Class>> {
        self.class_name_map.get(class_name)
    }

    pub fn complete_type(&self, word: &str) -> Option<Vec<String>> {
        let candidates = self.type_completer.complete(word);
        return (!candidates.is_empty()).then_some(candidates);
    }
    pub fn complete_var(&self, word: &str) -> Option<Vec<String>> {
        let candidates = self.var_completer.complete(word);
        return (!candidates.is_empty()).then_some(candidates);
    }
}

// --- --- --- --- --- ---

fn get_assignment_node(expression_node: &ExpressionNode) -> Option<&AssignmentNode> {
    let first_node = expression_node.elements.get(0)?;
    let ASTNode::Assignment(assignment_node) = first_node else {
        return None;
    };
    return Some(&assignment_node);
}

fn get_func_def_node(expression_node: &ExpressionNode) -> Option<&FunctionDefinitionNode> {
    let assignment_node = get_assignment_node(expression_node)?;
    let first_right_value_node = assignment_node.right_hand_node.elements.get(0)?;
    let ASTNode::FunctionDefinition(func_def_node) = first_right_value_node else {
        return None;
    };
    return Some(func_def_node);
}

// --- --- --- --- --- ---

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

fn analyze_scope(node_list: Vec<ASTNode>, scope: &mut EditorScope) {
    for node in node_list {
        if let ASTNode::Statement(ref statement_node) = node {
            if let StatementNode::Import(import_node) = statement_node.as_ref() {
                scope.import_std(&import_node.target);
                continue;
            }
        }

        let ASTNode::Expression(expression_node) = node else { continue };
        let Some(assignment_node) = get_assignment_node(&expression_node) else { continue };
        let ASTNode::Variable(var_node) = &assignment_node.left_hand_node else { continue };

        scope.insert_var(&var_node.name);

        match &assignment_node.right_hand_node.elements[0] {
            ASTNode::ClassDefinition(class_def_node) => {
                let class_inst = Class::from(class_def_node.as_ref());
                scope.insert_class(&var_node.name, class_inst);
            }
            ASTNode::Instantiation(instantiation_node) => {
                scope.insert_obj(&var_node.name, &instantiation_node.class);
            }
            ASTNode::ImportStatement(import_node) => {
                scope.import_from_path(&import_node.target);
            }
            _ => {}
        }
    }
}

fn analyze_global(node_list: Vec<RootNode>, scope: &mut EditorScope) {
    let node_list = node_list
        .into_iter()
        .map(|root_node| root_node.sub_node)
        .collect();
    analyze_scope(node_list, scope);
}

fn analyze_local(local_node: Option<&FunctionDefinitionNode>, scope: &mut EditorScope) {
    // for local scope, first to get the nearest function definition node
    let mut nearest_func_def_node = local_node;
    while let Some(func_def_node) = nearest_func_def_node {
        let last_node = func_def_node
            .body
            .iter()
            .rev()
            .find(|node| **node != ASTNode::Comment); // skip the last comment node
        let Some(ASTNode::Expression(expression_node)) = last_node else {
            break;
        };
        nearest_func_def_node = get_func_def_node(expression_node);
    }

    let Some(func_def_node) = nearest_func_def_node else {
        return;
    };
    // assign function params
    for param in &func_def_node.params {
        scope.insert_var(&param.identi);
    }
    // execute local codes
    let func_body_nodes = func_def_node.body
        .clone()
        .into_iter()
        .collect();
    analyze_scope(func_body_nodes, scope);
}

pub fn analyze(context_lines: Vec<String>, current_indent: usize) -> EditorScope {
    let mut scope = EditorScope::new();
    let context_lines = complete_brace(context_lines, current_indent);
    let mut ast_list = script_analyze(context_lines);
    if ast_list.is_empty() {
        return scope;
    }

    let last_node = ast_list.pop().unwrap();
    let (global_node_list, local_node) = match &last_node.sub_node {
        ASTNode::Expression(expression_node)
            if get_func_def_node(expression_node).is_some() => {
            let func_def_node = get_func_def_node(expression_node).unwrap();
            (ast_list, Some(func_def_node))
        }
        _ => {
            ast_list.push(last_node);
            (ast_list, None)
        }
    };

    // analyze global and local scope
    analyze_global(global_node_list, &mut scope);
    analyze_local(local_node, &mut scope);
    return scope;
}

use std::rc::Rc;

use crate::computer::resolvers::sequence;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn invoke(le_body: Rc<ASTNode>, scope: &mut Scope) -> Result<Value, ()> {
    // le -> lazy_expression
    let result = sequence::resolve(&le_body, scope)?;
    return Ok(result);
}

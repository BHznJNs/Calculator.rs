use std::rc::Rc;

use crate::computer::resolvers::sequence;
use crate::public::compile_time::ast::ast_enum::ASTNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

pub fn invoke(
    le_body: Rc<ASTNode>,
    scope: &mut Scope
) -> Result<Value, ()> {
    // let params = le_body
    //     .params
    //     .as_ref()
    //     .unwrap();
    // let expression_node = &params[0];

    let result =
        sequence::resolve(&mut le_body.as_ref(), scope)?;

    Ok(result)
}
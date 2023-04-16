use crate::computer::resolvers::sequence;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;
use crate::public::compile_time::ast::ASTNode;

pub fn invoke(
    le_body: &ASTNode,
    scope: &mut Scope
) -> Result<Value, ()> {
    /*
        lazy-expression body:
        {
            LazyExpression: {
                Expression: {
                    Number,
                    Symbol,
                    ...
                }
            }
        }
     */

    let params = le_body
        .params
        .as_ref()
        .unwrap();
    let expression_node = &params[0];

    let result =
        sequence::resolve(expression_node, scope)?;

    Ok(result)
}
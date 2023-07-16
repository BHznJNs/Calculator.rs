use crate::public::compile_time::ast::ast_enum::RootNode;
use crate::public::run_time::scope::Scope;
use crate::public::value::value::Value;

use super::resolvers::sequence;

pub fn compute(root_node: RootNode, scope: &mut Scope) -> Result<Value, ()> {
    /*
     Root {
       Expression {
         Assignment,
         Symbol,
         Number,
         Symbol,
         Expression,
         LazyExpression,
         ...
       },
       Statement {
         Keywords,
         Expression,
         ...
       }
     }
    */

    let sequence_node = root_node.sub_node;
    let result = sequence::resolve(&sequence_node, scope)?;

    return Ok(result);
}

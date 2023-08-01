use crate::public::{
    compile_time::ast::types::MapLiteralNode,
    run_time::scope::Scope,
    value::map::{RawMap, InternalMap},
};
use super::expression;

pub fn resolve(node: &MapLiteralNode, scope: &mut Scope) -> Result<RawMap, ()> {
    let mut internal_map = InternalMap::new();

    let mut key_iter = node.keys.iter();
    let mut expr_iter = node.values.iter();
    while let (Some(key), Some(expr)) = (key_iter.next(), expr_iter.next()) {
        let expr_result = expression::resolve(expr, scope)?;
        internal_map.insert(key.to_owned(), expr_result);
    }
    return Ok(RawMap::new(internal_map));
}

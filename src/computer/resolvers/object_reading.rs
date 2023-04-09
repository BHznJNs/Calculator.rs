use std::cell::{RefMut, Ref};
use std::rc::Rc;

use crate::public::compile_time::ast::{ASTNode, ASTNodeTypes};
use crate::public::run_time::scope::Scope;
use crate::public::value::oop::object::Object;
use crate::public::value::value::Value;

use super::variable_reading;

fn write(
    obj_ref: RefMut<Object>,
    value : Rc<Value>,
    params: &Vec<ASTNode>,
    index : usize,
) -> Result<(), ()> {
    let current = &params[index];
    let ASTNodeTypes::ObjectReading(prop_name) =
        &current.type__ else {
        println!("Invalid property name.");
        return Err(())
    };

    if index == params.len() - 1 {
        obj_ref.set(prop_name, value.unwrap())?;
        Ok(())
    } else {
        // nested reading
        let target_prop =
            obj_ref.get(prop_name)?;

        let Value::Object(obj_rc) =
            target_prop.as_ref() else {
            println!("'{}' in object is not a valid object.", prop_name);
            return Err(())
        };
        let obj_ref =
            obj_rc.as_ref().borrow_mut();
        Ok(write(obj_ref, value, params, index + 1)?)
    }
}

pub fn assign(
    node : &ASTNode,
    value: Rc<Value>,
    scope: &mut Scope,
) -> Result<(), ()> {
    let ASTNodeTypes::ObjectReading(obj_name) =
        &node.type__ else {
        println!("Invalid object name.");
        return Err(())
    };
    let params =
        node.params
        .as_ref()
        .unwrap();

    let var_value =
        variable_reading::resolve(obj_name, scope)?;
    let Value::Object(obj_rc) = var_value.as_ref() else {
        println!("'{}' is not a valid object.", obj_name);
        return Err(())
    };

    let obj_ref =
        obj_rc.as_ref().borrow_mut();

    Ok(write(obj_ref, value, params, 0)?)
}

// --- --- --- --- --- ---

fn read(
    obj_ref: Ref<Object>,
    params: &Vec<ASTNode>,
    index : usize,
) -> Result<Rc<Value>, ()> {
    let current = &params[index];
    let ASTNodeTypes::ObjectReading(prop_name) =
        &current.type__ else {
        println!("Invalid property name.");
        return Err(())
    };

    let target_prop =
        obj_ref.get(prop_name)?;

    if index == params.len() - 1 {
        Ok(target_prop)
    } else {
        // nested reading
        let Value::Object(obj_rc) =
            target_prop.as_ref() else {
            println!("'{}' in object is not a valid object.", prop_name);
            return Err(())
        };
        let obj_ref =
            obj_rc.as_ref().borrow();
        Ok(read(obj_ref, params, index + 1)?)
    }
}

pub fn resolve(
    node : &ASTNode,
    scope: &mut Scope,
) -> Result<Rc<Value>, ()> {
    let ASTNodeTypes::ObjectReading(obj_name) =
        &node.type__ else {
        println!("Invalid object name.");
        return Err(())
    };
    let params =
        node.params
        .as_ref()
        .unwrap();

    let var_value =
        variable_reading::resolve(obj_name, scope)?;
    let Value::Object(obj_rc) = var_value.as_ref() else {
        println!("'{}' is not a valid object.", obj_name);
        return Err(())
    };

    let obj_ref =
        obj_rc.as_ref().borrow();

    Ok(read(obj_ref, params, 0)?)
}
use std::{cell::Ref, str::FromStr};

use crate::public::error::{syntax_error, CalcResult};

#[inline]
pub fn str_to_num<T: FromStr>(str: Ref<String>) -> CalcResult<T> {
    // i64 || f64
    match str.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => Err(syntax_error("invalid string parse")),
    }
}

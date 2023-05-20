use std::{cell::Ref, str::FromStr};

pub fn str_to_num<T: FromStr>(
    str: Ref<String>
) -> Result<T, ()> {
    // i64 || f64
    match str.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => {
            println!("Invalid string coverting to number.");
            return Err(())
        },
    }
}
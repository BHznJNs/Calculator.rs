use std::fs::{self, File};

use crate::public::error::{internal_error, InternalComponent, CalcResult};

pub fn file_create(path: &str) -> CalcResult<()> {
    match File::create(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("file '{}' create error", path);
            Err(internal_error(InternalComponent::Std, &msg))
        }
    }
}
pub fn dir_create(path: &str) -> CalcResult<()> {
    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("folder '{}' create error", path);
            Err(internal_error(InternalComponent::Std, &msg))
        }
    }
}

pub fn dir_delete(path: &str) -> CalcResult<()> {
    match fs::remove_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("folder '{}' delete error", path);
            Err(internal_error(InternalComponent::Std, &msg))
        }
    }
}
pub fn file_delete(path: &str) -> CalcResult<()> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("file '{}' delete error", path);
            Err(internal_error(InternalComponent::Std, &msg))
        }
    }
}

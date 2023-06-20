use std::fs::{File, self};

use crate::public::error::{InternalComponent, internal_error};

pub fn file_create(path: &str) -> Result<(), ()> {
    match File::create(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("file '{}' create error", path);
            internal_error(InternalComponent::Std, &msg)
        },
    }
}
pub fn dir_create(path: &str) -> Result<(), ()> {
    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("folder '{}' create error", path);
            internal_error(InternalComponent::Std, &msg)
        },
    }
}

pub fn dir_delete(path: &str) -> Result<(), ()> {
    match fs::remove_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("folder '{}' delete error", path);
            internal_error(InternalComponent::Std, &msg)
        },
    }
}
pub fn file_delete(path: &str) -> Result<(), ()> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let msg = format!("file '{}' delete error", path);
            internal_error(InternalComponent::Std, &msg)
        },
    }
}
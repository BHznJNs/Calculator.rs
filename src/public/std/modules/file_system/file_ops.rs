use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use crate::public::{
    error::{internal_error, InternalComponent},
    value::{
        array::ArrayLiteral,
        value::Value,
    },
};

const TRUE_VALUE: Value = Value::Boolean(true);

pub fn file_read(file_path: &str, file_info: (Value, Value, Value)) -> Result<Value, ()> {
    let (exist, is_dir, is_file) = file_info;

    if exist == TRUE_VALUE {
        if is_file == TRUE_VALUE {
            let mut file = File::open(file_path).unwrap();
            let mut buffer = String::new();
            if file.read_to_string(&mut buffer).is_err() {
                let msg = format!("file '{}' read error", file_path);
                return Err(internal_error(InternalComponent::Std, &msg)?);
            }
            return Ok(Value::from(buffer));
        }
        if is_dir == TRUE_VALUE {
            let mut dir_content = ArrayLiteral::new();
            let sub_paths = fs::read_dir(file_path);

            if sub_paths.is_err() {
                let msg = format!("folder '{}' read error", file_path);
                return Err(internal_error(InternalComponent::Std, &msg)?);
            }

            for entry in sub_paths.unwrap() {
                let path = entry.unwrap().path();
                dir_content.push_back(Value::from(path.display().to_string()))
            }
            return Ok(Value::from(dir_content));
        }
        unreachable!()
    } else {
        let msg = format!("file '{}' does not exist", file_path);
        Err(internal_error(InternalComponent::Std, &msg)?)
    }
}

pub fn file_write(
    file_path: &str,
    content_value: Value,
    file_info: (Value, Value, Value),
) -> Result<(), ()> {
    let content_str = content_value.get_str()?;
    let (exist, _, is_file) = file_info;

    if exist == TRUE_VALUE && is_file == TRUE_VALUE {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();

        // clear file content
        file.set_len(0).unwrap();
        file.flush().unwrap();

        match file.write_all(content_str.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => {
                let msg = format!("file '{}' is not writable", file_path);
                Err(internal_error(InternalComponent::Std, &msg)?)
            }
        }
    } else {
        let msg = format!("path '{}' is not a legal file", file_path);
        Err(internal_error(InternalComponent::Std, &msg)?)
    }
}
pub fn file_append(
    file_path: &str,
    content_value: Value,
    file_info: (Value, Value, Value),
) -> Result<(), ()> {
    let Value::String(content_ref) = content_value else {
        unreachable!()
    };
    let content_str_temp = content_ref.borrow();
    let content = content_str_temp.as_str();

    let (exist, _, is_file) = file_info;
    if exist == TRUE_VALUE && is_file == TRUE_VALUE {
        let mut file = OpenOptions::new().append(true).open(file_path).unwrap();

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => {
                let msg = format!("file '{}' is not writable", file_path);
                Err(internal_error(InternalComponent::Std, &msg)?)
            }
        }
    } else {
        let msg = format!("path '{}' is not a legal file", file_path);
        Err(internal_error(InternalComponent::Std, &msg)?)
    }
}

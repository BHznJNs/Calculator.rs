use std::collections::HashMap;

use super::data_storage::DataStoragePattern;

pub fn getter<T: Clone>(
    storage_pattern: DataStoragePattern,
    target_prop: &str,
    list: &Option<Vec<(String, T)>>,
    map: &Option<HashMap<String, T>>,
) -> Result<T, String> {
    match storage_pattern {
        DataStoragePattern::List => {
            let data_list = list.as_ref().unwrap();

            for data_tuple in data_list {
                if target_prop.eq(&data_tuple.0) {
                    let target_value = &data_tuple.1;
                    return Ok(target_value.clone());
                }
            }
            Err(format!("Property '{}' does not exist.", target_prop))
        }
        DataStoragePattern::Map => {
            let data_map = map.as_ref().unwrap();

            match data_map.get(target_prop) {
                Some(target_value) => Ok(target_value.clone()),
                None => Err(format!("Property '{}' does not exist.", target_prop)),
            }
        }
    }
}

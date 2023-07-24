use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum DataStoragePattern {
    List,
    Map,
}
impl DataStoragePattern {
    const STORAGE_THRESHOLD: usize = 8;
}

// --- --- --- --- --- ---

pub type ListStorage<Item> = Vec<(String, Item)>;
pub type MapStorage<Item> = HashMap<String, Item>;

#[derive(PartialEq, Clone)]
pub struct ComposeStorage<Item> {
    pub(super) storage_pattern: DataStoragePattern,
    pub(super) data_list: Option<ListStorage<Item>>,
    pub(super) data_map: Option<MapStorage<Item>>,
}

impl<Item: Clone> ComposeStorage<Item> {
    pub fn new(items: Vec<(String, Item)>) -> Self {
        let storage_pattern = if items.len() > DataStoragePattern::STORAGE_THRESHOLD {
            DataStoragePattern::Map
        } else {
            DataStoragePattern::List
        };

        let data_list: Option<ListStorage<Item>>;
        let data_map: Option<MapStorage<Item>>;

        match storage_pattern {
            DataStoragePattern::List => {
                data_list = Some(items);
                data_map = None;
            }
            DataStoragePattern::Map => {
                let mut temp_map = MapStorage::<Item>::new();
                for (k, v) in items {
                    temp_map.insert(k, v);
                }
                data_list = None;
                data_map = Some(temp_map);
            }
        }
        return ComposeStorage {
            storage_pattern,
            data_list,
            data_map,
        };
    }

    pub fn getter(&self, target_prop: &str) -> Result<Item, ()> {
        match self.storage_pattern {
            DataStoragePattern::List => {
                let data_list = self.data_list.as_ref().unwrap();
    
                for data_tuple in data_list {
                    if target_prop.eq(&data_tuple.0) {
                        let target_value = &data_tuple.1;
                        return Ok(target_value.clone());
                    }
                }
                Err(())
            }
            DataStoragePattern::Map => {
                let data_map = self.data_map.as_ref().unwrap();
    
                match data_map.get(target_prop) {
                    Some(target_value) => Ok(target_value.clone()),
                    None => Err(()),
                }
            }
        }
    }
    pub fn setter(&mut self, target_prop: &str, value: Item) -> Result<(), ()> {
        match self.storage_pattern {
            DataStoragePattern::List => {
                let data_list = self.data_list.as_mut().unwrap();

                for data_tuple in data_list {
                    if target_prop.eq(&data_tuple.0) {
                        data_tuple.1 = value;
                        return Ok(());
                    }
                }
                Err(())
            }
            DataStoragePattern::Map => {
                let data_map = self.data_map.as_mut().unwrap();

                match data_map.get_mut(target_prop) {
                    Some(target_value) => {
                        *target_value = value;
                        Ok(())
                    },
                    None => Err(()),
                }
            }
        }
    }
}

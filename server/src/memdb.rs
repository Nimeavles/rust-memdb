use std::collections::HashMap;

use crate::Command;

pub struct MemDb {
    hash_map: HashMap<String, String>,
}

impl MemDb {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn execute(&mut self, query: &str) -> Result<String, String> {
        match query.parse().unwrap() {
            Command::Set(key, value) => {
                return match self.hash_map.insert(key.to_owned(), value.to_owned()) {
                    Some(old_value) => Ok(format!("Updated value from {old_value} to {value}")),
                    None => Ok(String::from("Ok: Query inserted successfully!")),
                }
            }
            Command::Get(key) => {
                if let Some(value) = self.hash_map.get(key.as_str()) {
                    return Ok(value.to_string());
                }

                Err(String::from(format!("Error: {key} doesn't exist!")))
            }
            Command::Del(key) => {
                if let Some(_) = self.hash_map.get(&key) {
                    self.hash_map
                        .remove(&key)
                        .expect("Error: Fatal Error deleting the key");
                    return Ok(String::from("Ok: Query deleted successfully"));
                }

                Err(String::from(format!("Error: {key} doesn't exist!")))
            }
        }
    }
}

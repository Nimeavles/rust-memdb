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

#[cfg(test)]
mod test {
    use crate::MemDb;

    #[test]
    fn test_set_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("set car ferrari"),
            Ok("Ok: Query inserted successfully!".to_string())
        );
    }

    #[test]
    fn test_update_value_with_set_command() {
        let mut memdb = MemDb::new();

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(
            memdb.execute("set car toyota"),
            Ok("Updated value from ferrari to toyota".to_string())
        );
    }

    #[test]
    fn test_get_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("get car"),
            Err("Error: car doesn't exist!".to_string())
        );

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(memdb.execute("get car"), Ok("ferrari".to_string()));
    }

    #[test]
    fn test_del_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("del car"),
            Err("Error: car doesn't exist!".to_string())
        );

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(
            memdb.execute("del car"),
            Ok("Ok: Query deleted successfully".to_string())
        );
    }
}

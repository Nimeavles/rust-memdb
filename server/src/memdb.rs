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
        match query.parse() {
            Ok(command) => {
                return match command {
                    Command::Set(key, value) => {
                        return match self.hash_map.insert(key.to_owned(), value.to_owned()) {
                            Some(old_value) => {
                                Ok(format!("Ok: Updated value from {old_value} to {value}\n"))
                            }
                            None => Ok(String::from("Ok: Query inserted successfully!\n")),
                        }
                    }
                    Command::Get(key) => {
                        if let Some(value) = self.hash_map.get(key.as_str()) {
                            return Ok(format!("Ok: {value}\n"));
                        }

                        Err(String::from(format!("Error: {key} doesn't exist!\n")))
                    }
                    Command::Del(key) => {
                        if let Some(_) = self.hash_map.get(&key) {
                            self.hash_map
                                .remove(&key)
                                .expect("Error: Fatal Error deleting the key");
                            return Ok(String::from("Ok: Query deleted successfully\n"));
                        }

                        Err(String::from(format!("Error: {key} doesn't exist!\n")))
                    }
                }
            }
            Err(error) => Err(format!("Error: {error}\n")),
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
            Ok("Ok: Query inserted successfully!\n".to_string())
        );
    }

    #[test]
    fn test_update_value_with_set_command() {
        let mut memdb = MemDb::new();

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(
            memdb.execute("set car toyota"),
            Ok("Ok: Updated value from ferrari to toyota\n".to_string())
        );
    }

    #[test]
    fn test_get_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("get car"),
            Err("Error: car doesn't exist!\n".to_string())
        );

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(memdb.execute("get car"), Ok("Ok: ferrari\n".to_string()));
    }

    #[test]
    fn test_del_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("del car"),
            Err("Error: car doesn't exist!\n".to_string())
        );

        memdb.execute("set car ferrari").unwrap();

        assert_eq!(
            memdb.execute("del car"),
            Ok("Ok: Query deleted successfully\n".to_string())
        );
    }

    #[test]
    fn test_failure_query() {
        let mut memdb = MemDb::new();

        assert_eq!(
            memdb.execute("invalid_method rust"),
            Err("Error: Unexpected method invalid_method!\n".to_string())
        )
    }
}

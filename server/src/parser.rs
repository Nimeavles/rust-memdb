use crate::query::{Command, Query};

// Returns the method of the query
fn parse_query_method(query: &str) -> Result<String, &str> {
    if query.starts_with('"') {
        return match query[1..].find("\"") {
            Some(index) => Ok(String::from(&query[1..index + 1])),
            None => Err("Expected string termination"),
        };
    }

    match query.find(|c: char| c.eq(&' ') || c.eq(&'"')) {
        Some(index) => {
            if query[index..index + 1].eq("\"") {
                Err("Unexpected string initializer")
            } else {
                Ok(String::from(&query[0..index]))
            }
        }
        None => Ok(String::from(query)),
    }
}

fn split_throught_ws(query: &str, query_struct: &mut Query) {
    let splitted: Vec<&str> = query.split(" ").collect();

    query_struct.key = splitted.get(0).expect("Invalid Query!").to_string();

    if let Some(value) = splitted.get(1) {
        query_struct.value = Some(value.to_string());
    }
}

fn parse_key_only(query_struct: &Query) -> Result<Command, String> {
    match query_struct.command.as_str() {
        "get" => Ok(Command::Get(query_struct.key.clone())),
        "del" => Ok(Command::Del(query_struct.key.clone())),
        &_ => Err(format!("Unexpected method {}!", query_struct.command)),
    }
}

pub fn parse(query: &str) -> Result<Command, String> {
    let mut query_struct = Query::new();
    query_struct.command = parse_query_method(query)?;

    // Remove the method and the following ws
    if query.len() < 4 {
        return Err("Invalid query!".to_string());
    }

    let new_query = &query[4..];
    split_throught_ws(new_query, &mut query_struct);

    match query_struct.value {
        Some(value) => {
            if query_struct.command.clone() == "set".to_string() {
                return Ok(Command::Set(query_struct.key, value));
            } else {
                Err(format!("Unexpected token: {value}"))
            }
        }
        None => parse_key_only(&query_struct),
    }
}

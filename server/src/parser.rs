use crate::query::{Command, Query};

// Returns the method of the query
fn parse_query_method(query: &str) -> Result<(String, usize), &str> {
    if query.starts_with('"') {
        return match query[1..].find("\"") {
            Some(index) => Ok((String::from(&query[1..index]), query[1..index].len())),
            None => Err("Expected string termination"),
        };
    }

    match query.find(|c: char| c.eq(&' ') || c.eq(&'"')) {
        Some(index) => {
            if query[index..index + 1].eq("\"") {
                Err("Unexpected string initializer")
            } else {
                Ok((String::from(&query[0..index]), query[0..index].len()))
            }
        }
        None => Ok((String::from(query), query.len())),
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
    if query.is_empty() {
        return Err(String::from("Empty query!"));
    }

    let mut query_struct = Query::new();

    match parse_query_method(query) {
        Ok((command, len)) => {
            query_struct.command = command;

            // Remove the method and the following ws
            if query.len() < len {
                return Err("Invalid query!".to_string());
            }

            let new_query = &query[len + 1..];
            split_throught_ws(new_query, &mut query_struct);
        }

        Err(err) => return Err(err.to_string()),
    };

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

#[cfg(test)]
mod test {
    use super::parse;
    use crate::Command;

    #[test]
    fn parse_set_query() {
        assert_eq!(
            Command::Set("car".to_string(), "ferrari".to_string()),
            parse("set car ferrari").unwrap()
        );

        assert_ne!(
            Command::Set("car".to_string(), "ferrari".to_string()),
            parse("set car lambo").unwrap()
        );
    }

    #[test]
    fn parse_get_query() {
        assert_eq!(Command::Get("car".to_string()), parse("get car").unwrap());

        assert_ne!(Command::Get("tv".to_string()), parse("get sofa").unwrap());
    }

    #[test]
    #[should_panic]
    fn parse_bad_get_query() {
        std::panic::set_hook(Box::new(|_info| {
            // In order to not to show any panic info
        }));
        parse("get car ferrari").unwrap();
    }

    #[test]
    fn parse_del_query() {
        assert_eq!(Command::Del("car".to_string()), parse("del car").unwrap());
        assert_ne!(
            Command::Del("pillow".to_string()),
            parse("del car").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn parse_bad_del_query() {
        std::panic::set_hook(Box::new(|_info| {
            // In order to not to show any panic info
        }));
        parse("del car ferrari").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_invalid_query() {
        std::panic::set_hook(Box::new(|_info| {
            // In order to not to show any panic info
        }));
        parse("invalid_query rust").unwrap();
    }
}

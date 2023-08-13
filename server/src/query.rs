use std::str::FromStr;

use crate::parse;

#[derive(Debug)]
pub struct Query {
    pub command: String,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
}

impl Query {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            key: String::new(),
            value: None,
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

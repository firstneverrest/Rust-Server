use std::collections::HashMap;
use std::convert::From; // not throw error when it is not existed

#[derive(std::fmt::Debug)] // implement debug automatically

pub struct QueryString {
    data: HashMap<String, String>, // store key value pairs
}

impl QueryString {
    pub fn get(&self, key: &String) -> Option<&String> {
        self.data.get(key)
    }
}

impl From<&str> for QueryString {
    fn from(s: &str) -> Self {
        let mut data = HashMap::new();

        // name=carlos&salary=30000
        for c in s.split('&') {
            if let Some(i) = c.find('=') {
                let key = &c[..i];
                let value = &c[i+1..];
                data.insert(key.to_string(), value.to_string());
            }
        }

        QueryString { data }
    }
}
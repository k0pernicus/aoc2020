pub mod parser;

use regex::Regex;
use std::collections::HashMap;
use std::default;

pub struct Rules {
    map: HashMap<usize, String>,
}

impl Rules {
    pub fn is_match(&self, id: usize, s: &str) -> bool {
        if !self.map.contains_key(&id) {
            return false;
        }
        let re_s = format!("^{}$", self.map.get(&id).unwrap());
        match Regex::new(re_s.as_str()) {
            Ok(re) => re.is_match(s),
            Err(err) => {
                println!("Error when constructing the regex: {}", err);
                return false;
            }
        }
    }
    pub fn get_rule(&self, id: usize) -> Regex {
        let re_s = format!("^{}$", self.map.get(&id).unwrap());
        Regex::new(re_s.as_str()).unwrap()
    }
    pub fn drop(self) {}
}

impl default::Default for Rules {
    fn default() -> Self {
        Rules {
            map: HashMap::new(),
        }
    }
}

macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

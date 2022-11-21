use std::env;
use std::collections::HashMap;

pub struct Config {
    pub prompt: String,
    pub aliases: HashMap<String, String>,
    pub startup: Vec<String>,
    pub history_cap: usize,
}

impl Config {
    pub fn new() -> Config {
        let mut alias: HashMap<String, String> = HashMap::new();
        alias.insert(String::from("ls"), String::from("ls --color=auto"));

        return Config {
            prompt: format!("{}$ ", env::current_dir().unwrap().to_str().unwrap()),
            aliases: alias,
            startup: vec![String::from("colorscript --random")],
            history_cap: 100,
        };
    }
}




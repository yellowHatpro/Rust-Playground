use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    let res = if config.is_case_sensitive {
        search( &config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    };
    for line in res {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub pattern: String,
    pub path: String,
    pub is_case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let pattern = args[1].clone();
        let path = args[2].clone();
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            pattern,
            path,
            is_case_sensitive
        })
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            res.push(line);
        }
    }
    res
}

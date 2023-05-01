use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config {
            query,
            filename,
            sensitive: false,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.sensitive {
        for line in search_sensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

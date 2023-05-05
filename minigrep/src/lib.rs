use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename string"),
        };

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
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

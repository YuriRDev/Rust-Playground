use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::*;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("[err] Problem parsing arguments: {}", err);
        process::exit(0);
    });

    match run(config) {
        Err(e) => {
            println!("Application error: {}", e)
        },
        _ => {}
    }
}


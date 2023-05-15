use highlight_search::*;
use std::env;
use std::process;

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
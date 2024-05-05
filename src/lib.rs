mod config;
mod search;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub use config::CommandLineConfig;
pub use search::{search, search_case_insensitive};

pub fn minigrep() {
    let args: Vec<String> = env::args().collect();
    let config = CommandLineConfig::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: CommandLineConfig) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename())?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive() {
        search(&contents, config.query())
    } else {
        search_case_insensitive(&contents, config.query())
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod test;

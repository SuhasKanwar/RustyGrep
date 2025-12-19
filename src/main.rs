use std::env;
use std::fs;
use std::process;
use std::error::Error;
use rusty_grep::{ search, search_case_insensitive };

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!!!")
        };
        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path!!!")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case
        })
    }
}

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &'static str| {
        eprintln!("Problem parsing the arguments -> {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error -> {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
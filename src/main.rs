use std::env;
use std::fs;
use std::process;
use std::error::Error;
use rusty_grep::search;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!!!");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Self { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err: &'static str| {
        println!("Problem parsing the arguments -> {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error -> {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
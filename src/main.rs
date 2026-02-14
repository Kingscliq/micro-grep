use std::{env, error::Error, fs, process};

use microgrep::{case_insensitive_search, search};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("❌ An error occured while processing your request: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {e}")
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let query = config.query;
    let file_path = config.file_path;

    let contents = fs::read_to_string(file_path)?;

    let result = if config.ignore_case {
        case_insensitive_search(&query, &contents)
    } else {
        search(&query, &contents)
    };

    if result.len() < 1 {
        println!("\nSearch query not found!")
    }

    for line in result {
        println!("{line}")
    }

    Ok(())
}

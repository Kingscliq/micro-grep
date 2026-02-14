use std::{env, error::Error, fs, process};

use microgrep::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];

    // let input = parse_string(&args);

    // let query = config.query;
    // let file_path = config.file_path;
    // println!("Searching for {query}");
    // println!("In file {file_path}");

    // let contents = fs::read_to_string(file_path).expect("Maybe a the file does not exit");

    // println!("With text:\n{contents}");

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("❌ An error occured while processing your request: {err}");
    //     process::exit(1);
    // });

    // run(config);
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
    // println!("Searching for {query}");
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)?;

    for line in search(&query, &contents) {
        println!("{line}")
    }

    Ok(())
}

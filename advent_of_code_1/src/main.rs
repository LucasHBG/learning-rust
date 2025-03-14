use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching file path with query: {}", config.query);
    println!("In file {}", config.file_path);

    let _contents: String =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("With content: \n - {contents}");

    // while contents. {

    // }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config { query, file_path };
}

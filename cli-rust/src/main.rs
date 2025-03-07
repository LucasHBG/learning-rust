use std::{env, path};
// use clap::Parser;

// #[derive(Parser)]
struct Cli {
    /// The pattern to look for
    // #[arg(short, long)]
    pattern: String,
    /// The path to the file to read
    // #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() -> () {
    
    let query: String = env::args().nth(1).expect("no pattern given");
    let file_path: String = env::args().nth(2).expect("no path given");
    
    let args = Cli {
        pattern: query,
        path: path::PathBuf::from(file_path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    
}

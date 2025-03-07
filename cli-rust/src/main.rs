use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'q', long)]
    query: String,
    
    /// The path to the file to read
    #[arg(short = 'p', long)]
    path: std::path::PathBuf,

    /// lancei o tadala?
    #[arg(short = 't', long)]
    tadala: i8,
}

fn main() -> () {
    
    let args = Cli::parse();

    if args.tadala == 1 {
        println!("To pronto pro abate fellas!!");
    } else {
        println!("To broxa fellas :(");
    }

    println!("pattern: {:?}, path: {:?}, turbo: {:?}", args.query, args.path, args.tadala);

    
}

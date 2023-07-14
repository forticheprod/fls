use clap::Parser;
use framels::{basic_listing, extended_listing, parse_dir, recursive_dir};

/// Command line to list directory and pack frames in sequences
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Display EXR metadata size and channels description
    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    recursive: bool,

    /// Path to parse
    #[arg(default_value_t = String::from("./"), last = true)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let paths: Vec<String> = if args.recursive{
        recursive_dir(&args.path)
    }else{
        parse_dir(&args.path)
    };
    let results = if args.list {
        extended_listing(args.path, paths)
    } else {
        basic_listing(paths)
    };

    println!("{}", results.join("\n"))
}

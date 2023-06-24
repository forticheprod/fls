use clap::Parser;
use framels::{basic_listing, extended_listing, parse_dir};

/// Command line to list directory and pack frames by in sequence
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Display EXR metadata size and channels description
    #[arg(short, long)]
    list: bool,

    /// Path to parse
    #[arg(default_value_t = String::from("./"), last = true)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let results = if args.list {
        let paths: Vec<String> = parse_dir(args.path.clone());
        extended_listing(args.path, paths)
    } else {
        let paths: Vec<String> = parse_dir(args.path);
        basic_listing(paths)
    };

    println!("{}", results.join("\n"))
}

use clap::Parser;
use framels::{basic, listing, parse_dir};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    list: bool,

    #[arg(default_value_t = String::from("."), last = true)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let results = if args.list {
        let paths: Vec<String> = parse_dir(args.path.clone());
        listing(args.path, paths)
    } else {
        let paths: Vec<String> = parse_dir(args.path);
        basic(paths)
    };

    println!("{}", results.join("\n"))
}

use clap::Parser;
use framels::{basic_listing, extended_listing, parse_dir, paths::Paths, recursive_dir};

/// Command line to list directory and pack frames in sequences
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Display EXR metadata size and channels description
    #[arg(short, long)]
    list: bool,

    /// Use a recursive approch of listing dir
    #[arg(short, long)]
    recursive: bool,

    /// Represent output as a tree
    #[arg(short, long)]
    tree: bool,

    /// Path to parse
    #[arg(default_value_t = String::from("./"), last = true)]
    root: String,
}

fn main() {
    let args = Args::parse();
    let in_paths: Paths = if args.recursive && args.tree {
        recursive_dir(&args.root)
    } else {
        parse_dir(&args.root)
    };
    let results: String = if args.list && args.recursive {
        extended_listing("".to_string(), in_paths).join("\n")
    } else if args.list {
        extended_listing(args.root, in_paths).join("\n")
    } else {
        basic_listing(in_paths).get_paths().join("\n")
    };

    println!("{results}")
}

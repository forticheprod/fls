use std::path::PathBuf;

use clap::Parser;
use framels::{
    basic_listing, extended_listing, parse_dir,
    paths::{Join, Paths},
    recursive_dir,
};
mod tree;
use tree::run_tree;

/// Command line to list directory and pack frames in sequences
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Display EXR metadata size and channels description
    #[arg(short, long)]
    list: bool,

    /// Display EXR metadata size and channels description for a file
    #[arg(short, long)]
    exr: bool,

    /// Use a recursive approch of listing dir
    #[arg(short, long)]
    recursive: bool,

    /// Represent output as a tree
    #[arg(short, long)]
    tree: bool,

    /// Force the use of multithreading
    #[arg(short, long, default_value_t = false)]
    multithread: bool,

    /// Path to parse
    #[arg(value_name = "PATH", default_value_t = String::from("./"))]
    root: String,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Perform directory listing
    let in_paths: Paths = if args.exr {
        Paths::from(vec![PathBuf::from(&args.root)])
    } else if args.recursive {
        recursive_dir(&args.root)
    } else {
        parse_dir(&args.root)
    };
    let results = if args.list && args.recursive {
        extended_listing("".to_string(), in_paths, args.multithread)
    } else if args.list && args.exr {
        extended_listing("".to_string(), in_paths, args.multithread)
    } else if args.list {
        extended_listing(args.root, in_paths, args.multithread)
    } else {
        basic_listing(in_paths, args.multithread)
    };
    if args.tree {
        run_tree(results.get_paths().to_vec())
    } else {
        println!("{}", results.join("\n"))
    }
}

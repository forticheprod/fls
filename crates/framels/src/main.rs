use std::path::PathBuf;

use clap::Parser;
use framels::{extended_listing, parse_dir, recursive_dir, PathsPackedMeta};
use framels_core::{Paths, PathsPacked};
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
    let in_paths = if args.exr {
        Paths::from(vec![PathBuf::from(&args.root)])
    } else if args.recursive {
        recursive_dir(&args.root)
    } else {
        parse_dir(&args.root)
    };

    // Choose listing function based on arguments
    let results = if args.list {
        extended_listing(
            if args.recursive || args.exr {
                "".to_string()
            } else {
                args.root
            },
            in_paths,
            args.multithread,
        )
    } else {
        let p: PathsPacked = in_paths.pack(args.multithread);
        PathsPackedMeta::from(p, Vec::new())
    };

    // Display results based on arguments
    if args.tree {
        run_tree(results.get_paths().to_vec())
    } else {
        println!("{}", results.join("\n"))
    }
}

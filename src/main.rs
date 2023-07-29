use clap::Parser;
use comfy_table::Table;
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

    /// Use a recursive approch of listing dir
    #[arg(short, long)]
    table: bool,

    /// Path to parse
    #[arg(default_value_t = String::from("./"), last = true)]
    root: String,
}

fn main() {
    let mut table = Table::new();
    let args = Args::parse();
    let in_paths: Paths = if args.recursive {
        recursive_dir(&args.root)
    } else {
        parse_dir(&args.root)
    };
    let results: String = if args.list && args.recursive {
        extended_listing("".to_string(), in_paths).join("\n")
    } else if args.list && args.table {
        table.set_header(vec!["path"]);
        for i in extended_listing(args.root, in_paths).to_vec(){
            table.add_row(vec![i]);
        }
        table.to_string()
    } else if args.list {
        extended_listing(args.root, in_paths).join("\n")
    } else {
        basic_listing(in_paths).get_paths().join("\n")
    };

    println!("{results}")
}

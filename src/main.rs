use clap::Parser;
use framels::{parse_dir, run};

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
    println!("{}", &args.path);
    let paths: Vec<String> = parse_dir(args.path);
    let results: Vec<String> = run(paths);
    println!("{}", results.join("\n"))
}

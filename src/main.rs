use clap::Parser;
use framels::{parse_dir, run};
use std::env;
use std::path::Path;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'e')]
    exr: bool,

    #[arg(last = true)]
    path: String,
}

fn get_current_working_dir() -> String {
    let file_path = env::current_dir();
    match file_path {
        Ok(file) => file.to_string_lossy().to_string(),
        Err(_) => panic!("Current Dir does not exist"),
    }
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: String = if args.len() > 1 {
        args.get(1).unwrap().to_string()
    } else {
        get_current_working_dir()
    };

    let _ = Path::new(&file_path).exists();
    file_path.to_string()
}

fn main() {
    let args = Cli::parse();
    let paths: Vec<String> = parse_dir(get_path());
    let results: Vec<String> = run(paths);
    println!("{}", results.join("\n"))
}

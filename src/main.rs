use framels::{parse_dir, run};
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let input_paths = args.last();
    let paths: Vec<String> = parse_dir(input_paths);
    let results: Vec<String> = run(paths);
    println!("{}", results.join("\n"))
}

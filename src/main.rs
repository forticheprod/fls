use fil_ls::parse_result;
use std::collections::HashMap;
use std::fs::ReadDir;
use std::path::Path;
use std::{env, fs};

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let _ = Path::new(file_path).exists();
    println!("In path {}", file_path);
    file_path.to_string()
}

fn parse_dir(input_path: String) -> Vec<String> {
    let paths: ReadDir = fs::read_dir(input_path).unwrap();
    paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| s.to_string()))
            })
        })
        .collect::<Vec<String>>()
}

#[test]
fn test_parse_dir() {
    assert_eq!(5, crate::parse_dir("./samples".to_string()).len());
}

fn main() {
    let paths: Vec<String> = parse_dir(get_path());
    let results: HashMap<String, Vec<String>> = parse_result(paths);
    println!("{:#?}", results);
}

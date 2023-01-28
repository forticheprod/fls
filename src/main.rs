use std::path::Path;
use std::{env, fs};
mod lib;

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let _ = Path::new(file_path).exists();
    println!("In path {}", file_path);
    file_path.to_string()
}

fn parse_dir(input_path: String) -> Vec<String> {
    let paths = fs::read_dir(input_path).unwrap();
    paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>()
}

fn main() {
    let paths = parse_dir(get_path());
    println!("{:#?}", paths);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_dir() {
        assert_eq!(5, crate::parse_dir("./samples".to_string()).len());
    }
}

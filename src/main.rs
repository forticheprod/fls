use std::path::Path;
use std::{env, fs};

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let _ = Path::new(file_path).exists();
    println!("In file {}", file_path);
    file_path.to_string()
}

fn parse_dir(input_path: String) -> Vec<Vec<String>> {
    let mut paths_splited: Vec<Vec<String>> = Vec::new();
    let paths = fs::read_dir(&input_path).unwrap();
    for path in paths {
        let path_str =  path.unwrap().path().display().to_string();
        let vec:Vec<String> = path_str.split(".").map(|s| s.to_string()).collect();
        paths_splited.push(vec)
    }
    paths_splited
}

fn main() {
    let paths: Vec<Vec<String>> = parse_dir(get_path());
    println!("{}",paths.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_dir() {
        assert_eq!(5, super::parse_dir("./samples".to_string()).len());
    }
}

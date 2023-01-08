use std::path::Path;
use std::{env, fs};

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let _ = Path::new(file_path).exists();
    println!("In file {}", file_path);
    file_path.to_string()
}

fn parse_dir(input_path: String) {
    let mut paths_splited: Vec<Vec<String>> = Vec::new();
    let paths = fs::read_dir(&input_path).unwrap();
    for path in paths {
        let path_str =  path.unwrap().path().display().to_string();
        let split = path_str.split(".");
        let vec = split.collect::<Vec<&str>>();
        paths_splited.push(vec)
    }
}

fn main() {
    parse_dir(get_path())
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}

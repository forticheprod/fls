use std::{env, fs};
use std::path::PathBuf;
use expanduser::expanduser;

fn get_path()-> String {
    let args: Vec<String> = env::args().collect();
    let raw_path = &args[1];
    let path = expanduser(raw_path);
    let file_path = path.display().to_string();

    println!("In file {}", file_path);
    file_path.to_string()
}

fn main() {
    let input_path: &str = &get_path();
    let paths= fs::read_dir(input_path).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

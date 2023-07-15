//! # framels
//! framels is a Library to list files and directorys like `ls` style and return a packed
//! sequence of files. This lib is industry oriented for Animation and VFX,
//! using a lot a frames sequences.
//! The main objective is to be the fastest as possible using rustlang.
mod exr_metadata;
pub mod paths;
use crate::exr_metadata::read_meta;
use jwalk::WalkDir;
use paths::{Paths, PathsPacked};
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

/// # parse_dir
/// List files and directories in the targeted directory, take a `String` as
/// input and return a `Vec<String>` of the entries.
pub fn parse_dir(input_path: &String) -> paths::Paths {
    let paths_dir: fs::ReadDir = fs::read_dir(input_path).unwrap();
    let paths = paths_dir
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| s.to_string()))
            })
        })
        .collect::<Vec<String>>();
    Paths::new(paths)
}

/// # Recursive walking
/// List files and directories in the targeted directory, take a `String` as
/// inut and return a `Vec<String>` of the entries recursively
pub fn recursive_dir(input_path: &String) -> paths::Paths {
    let paths = WalkDir::new(input_path)
        .sort(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    Paths::new(paths)
}

/// This function compile the main regular expression used to extract the
/// frames from the file name.
/// (?x) match the remainder of the pattern with the following effective flags:
/// gmx x modifier: extended. Spaces and text after a # in the pattern are ignored
/// The frames group should contain between **2** and **9** digit, the extension
/// group should contains only letters between 2 and 5
fn get_regex() -> regex::Regex {
    let re = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$");
    match re {
        Ok(succes_value) => succes_value,
        Err(err) => panic!("Can't compile regex with error {}", err),
    }
}

/// This function extract the matching group based on regex already compile to
/// a tuple of string. For exemple toto.458.jpg should return
/// (toto.***.jpg, 458)
fn extract_regex(re: &Regex, x: String) -> (String, String) {
    let result_caps: Option<Captures> = re.captures(&x);
    match result_caps {
        None => (x, "None".to_string()),
        caps_wrap => {
            let caps = caps_wrap.unwrap();
            (
                x.replace(
                    &caps["frames"],
                    String::from_utf8(vec![b'*'; caps["frames"].len()])
                        .unwrap()
                        .as_str(),
                ),
                caps["frames"].to_string(),
            )
        }
    }
}

/// Parse the result of a vector of string. This function use HashMap to pack
/// filename removed from the frame value.
fn parse_result(dir_scan: Paths) -> HashMap<String, Vec<String>> {
    let re = get_regex();
    // Optimisation over PAR_THRESHOLD value, the parsing of the frame list
    // used rayon lib to paralelize the work. Result depends a lot from the
    // cpu number of thread may be put in a config file
    const PAR_THRESHOLD: usize = 100000;
    let extracted: Vec<(String, String)> = if dir_scan.len() < PAR_THRESHOLD {
        dir_scan
            .iter()
            .map(|path| extract_regex(&re, path.to_string()))
            .collect()
    } else {
        dir_scan
            .par_iter()
            .map(|path| extract_regex(&re, path.to_string()))
            .collect()
    };
    let mut paths_dict: HashMap<String, Vec<String>> = HashMap::new();
    for extraction in extracted {
        let vec1: Vec<String> = vec![extraction.1.clone()];
        paths_dict
            .entry(extraction.0)
            .and_modify(|value| (*value).push(extraction.1))
            .or_insert(vec1);
    }
    paths_dict
}

/// Convert the frame string to a number usefull to work with
/// fn group_continuity and apply comparaison of value
fn convert_vec(frames_vec: Vec<String>) -> Vec<isize> {
    let mut out_vec: Vec<isize> = frames_vec
        .into_iter()
        .map(|x: String| x.parse::<isize>().unwrap())
        .collect();
    out_vec.sort();
    out_vec
}

/// Check the continuity of a numbers' serie
fn group_continuity(data: &[isize]) -> Vec<Vec<isize>> {
    let mut slice_start: usize = 0;
    let mut result: Vec<&[isize]> = Vec::new();
    for i in 1..data.len() {
        if data[i - 1] + 1 != data[i] {
            result.push(&data[slice_start..i]);
            slice_start = i;
        }
    }
    if !data.is_empty() {
        result.push(&data[slice_start..]);
    }
    result.iter().map(|x| x.to_vec()).collect()
}

/// Concatenation of continuity group in a string
fn convert_vec_to_str(input_vec: Vec<Vec<isize>>) -> String {
    let mut tmp_vec: Vec<String> = Vec::new();
    for x in input_vec {
        if x.len() == 1 {
            tmp_vec.push(x[0].to_string());
        } else {
            tmp_vec.push(format!("{}-{}", x.first().unwrap(), x.last().unwrap()))
        }
    }
    tmp_vec.join(",")
}

/// Basic function to:
/// - convert vector of string into vector of isize
/// - analyse the continuity
/// - convert group of continuity into a concat string
fn create_frame_string(value: Vec<String>) -> String {
    let converted_vec_isize: Vec<isize> = convert_vec(value);
    let group_continuity: Vec<Vec<isize>> = group_continuity(&converted_vec_isize);
    convert_vec_to_str(group_continuity)
}

/// ## Basic listing of the library
/// ### Description
///
/// This function is the main function of the library it use a list of
/// filename as in input and pack the frame sequences using a new filename
/// like `toto.***.jpg@158-179`
///
/// It take a `Vec<String>` of entries as an input
///  - Pack the frames
pub fn basic_listing(frames: Paths) -> PathsPacked {
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut out_frames: PathsPacked = PathsPacked::new_empty();
    for (key, value) in frames_dict {
        if value[0] == "None" && value.len() == 1 {
            out_frames.push_paths(key);
        } else {
            out_frames.push_paths(format!("{}@{}", key, create_frame_string(value)));
        }
    }
    out_frames
}

/// This function is intented to check if a file is an exr to call exr module
/// and print the exr metadata of the file
fn get_exr_metada(re: &Regex, root_path: &String, path: &String) -> String {
    if re.is_match(&path) {
        let path = format!("{}{}", root_path, path);
        read_meta(path)
    } else {
        "Not an exr".to_string()
    }
}

/// ## Extended function of the Library
/// ### Description
///
/// This function is specialize to analyse exr frames really similar to
/// `rvls -l`
///
/// It take a `Vec<String>` of entries as an input
///  - Pack the frames
///  - Print the metada if the sequence is an exr sequence
///  - Return a Vector of path packed
pub fn extended_listing(root_path: String, frames: Paths) -> PathsPacked {
    let re: Regex = Regex::new(r".*.exr$").unwrap();
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut out_frames: PathsPacked = PathsPacked::new_empty();
    for (key, value) in frames_dict {
        if value[0] == "None" && value.len() == 1 {
            out_frames.push_metadata(get_exr_metada(&re, &root_path, &key));
            out_frames.push_paths(key);
        } else {
            let to = value.first().unwrap();
            let from = String::from_utf8(vec![b'*'; to.len()]).unwrap();
            let new_path = &key.replace(&from, to);
            out_frames.push_metadata(get_exr_metada(&re, &root_path, &new_path));
            out_frames.push_paths(format!("{}@{}", key, create_frame_string(value)));
        }
    }
    out_frames
}

/// Unitest
#[test]
fn test_parse_dir() {
    let source = "./samples/small".to_string();
    assert_eq!(6, crate::parse_dir(&source).len());
}
#[test]
fn test_handle_none() {
    let re = get_regex();
    let source: String = "foobar.exr".to_string();
    let expected: (String, String) = (source.clone(), "None".to_string());
    assert_eq!(expected, extract_regex(&re, source))
}

#[test]
fn test_regex_simple() {
    let re = get_regex();
    let source: String = "RenderPass_Beauty_1_00000.exr".to_string();
    let expected: (String, String) = (
        "RenderPass_Beauty_1_*****.exr".to_string(),
        "00000".to_string(),
    );
    assert_eq!(expected, extract_regex(&re, source))
}
#[test]
fn test_parse_string() {
    let source: Paths = Paths::new(vec![
        "toto.001.tiff".to_string(),
        "toto.002.tiff".to_string(),
        "toto.003.tiff".to_string(),
        "foo.exr".to_string(),
    ]);
    let vec_toto: Vec<String> = vec!["001".to_string(), "002".to_string(), "003".to_string()];
    let vec_foo: Vec<String> = vec!["None".to_string()];
    let expected: HashMap<String, Vec<String>> = HashMap::from([
        ("toto.***.tiff".to_string(), vec_toto),
        ("foo.exr".to_string(), vec_foo),
    ]);
    assert_eq!(expected, parse_result(source));
}
#[test]
fn test_convert_vec() {
    let source: Vec<String> = vec!["001".to_string(), "005".to_string(), "003".to_string()];
    let expected: Vec<isize> = vec![1, 3, 5];
    assert_eq!(expected, convert_vec(source));
}
#[test]
fn test_continuity() {
    let source: Vec<isize> = vec![1, 2, 3, 5, 6, 7, 11, 12];
    let expected: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12]];
    assert_eq!(expected, group_continuity(&source));
}
#[test]
fn test_convert_vec_to_str() {
    let source: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12], vec![45]];
    let expected: String = "1-3,5-7,11-12,45".to_string();
    assert_eq!(expected, convert_vec_to_str(source));
}
#[test]
fn test_create_frame_string() {
    let source: Vec<String> = vec![
        "001".to_string(),
        "005".to_string(),
        "003".to_string(),
        "002".to_string(),
    ];
    let expected: String = "1-3,5".to_string();
    assert_eq!(expected, create_frame_string(source));
}

//! # framels
//! framels is a Library to list files and directorys like `ls` style and return a packed
//! sequence of files. This lib is industry oriented for Animation and VFX,
//! using a lot a frames sequences.
//! The main objective is to be the fastest as possible using rustlang.
//! ## Usage
//! ```rust
//! use framels::{basic_listing, extended_listing, parse_dir, paths::{Paths,Join}, recursive_dir};
//!
//! fn main() {
//!    // Perform directory listing
//!   let in_paths: Paths = parse_dir("./samples/small");
//!
//!  // Generate results based on arguments
//! let results: String = basic_listing(in_paths).get_paths().join("\n");
//!
//! println!("{}", results)
//! }
//! ```
//! ## Command line
//! framels is also a command line tool to list directory and pack frames in sequences
//! ```bash
//! framels
//! framels is a Library to list files and directorys like `ls` style and return a packed sequence of files
//! This lib is industry oriented for Animation and VFX, using a lot a frames sequences.
//! The main objective is to be the fastest as possible using rustlang.
//!     --list         Display EXR metadata size and channels description
//!    --recursive    Use a recursive approch of listing dir
//!   <root>    Path to parse
//! ```
//! ## Example
//! ```bash
//!     $ fls --list --recursive ./samples/small
//!    ./samples/small/RenderPass_Beauty_1_*****.exr@0-9    1920x1080, RGBA
//! ```
mod exr_metadata;
pub mod paths;
use crate::exr_metadata::read_meta;
use jwalk::WalkDir;
use lazy_static::lazy_static;
use paths::{New, Paths, PathsPacked};
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;
use std::{clone::Clone, path::PathBuf};

/// # parse_dir
/// List files and directories in the targeted directory, take a `String` as
/// input and return a `Vec<String>` of the entries.
pub fn parse_dir(input_path: &str) -> Paths {
    let paths_dir: fs::ReadDir = fs::read_dir(input_path).unwrap();
    Paths::from(
        paths_dir
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.path().file_name().map(PathBuf::from))
            })
            .collect::<Vec<PathBuf>>(),
    )
}

/// # Recursive walking
/// List files and directories in the targeted directory, take a `String` as
/// inut and return a `Vec<String>` of the entries recursively
pub fn recursive_dir(input_path: &str) -> Paths {
    Paths::from(
        WalkDir::new(input_path)
            .sort(true)
            .into_iter()
            .filter_map(|entry| entry.ok().and_then(|e| Some(e.path())))
            .collect::<Vec<PathBuf>>(),
    )
}

/// This function extract the matching group based on regex already compile to
/// a tuple of string. For exemple toto.458.jpg should return
/// (toto.***.jpg, 458)
#[inline(always)]
fn extract_regex(x: &str) -> (String, String) {
    lazy_static! {
        static ref RE_FLS: Regex = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$")
            .expect("Can't compile regex");
    }
    let result_caps: Option<Captures> = RE_FLS.captures(&x);
    match result_caps {
        None => (x.to_string(), "None".to_string()),
        caps_wrap => {
            let caps = caps_wrap.unwrap();
            (
                x.replace(&caps["frames"], &"*".repeat(caps["frames"].len())),
                caps["frames"].to_string(),
            )
        }
    }
}

/// Parse the result of a vector of string. This function use HashMap to pack
/// filename removed from the frame value.
fn parse_result(dir_scan: Paths) -> HashMap<String, Vec<String>> {
    // Optimisation over PAR_THRESHOLD value, the parsing of the frame list
    // used rayon lib to paralelize the work. Result depends a lot from the
    // cpu number of thread may be put in a config file
    const PAR_THRESHOLD: usize = 100000;
    let extracted: Vec<(String, String)> = if dir_scan.len() < PAR_THRESHOLD {
        dir_scan
            .iter()
            .map(|path| extract_regex(path.to_str().unwrap()))
            .collect()
    } else {
        dir_scan
            .par_iter()
            .map(|path| extract_regex(path.to_str().unwrap()))
            .collect()
    };
    let mut paths_dict: HashMap<String, Vec<String>> = HashMap::with_capacity(extracted.len());
    for extraction in extracted {
        let vec1: Vec<String> = vec![extraction.1.clone()];
        paths_dict
            .entry(extraction.0)
            .and_modify(|value| (*value).push(extraction.1))
            .or_insert(vec1);
    }
    paths_dict
}

/// Check the continuity of a numbers' serie and return a vector of vector of
/// isize with the continuity group
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

/// Basic function to:
/// - convert vector of string into vector of isize
/// - analyse the continuity
/// - convert group of continuity into a concat string
fn create_frame_string(value: Vec<String>) -> String {
    let mut converted_vec_isize: Vec<isize> = value
        .into_iter()
        .map(|x| x.parse().expect("Failed to parse integer"))
        .collect();
    converted_vec_isize.sort();
    let group_continuity: Vec<Vec<isize>> = group_continuity(&converted_vec_isize);
    // Concatenation of continuity group in a string
    group_continuity
        .into_iter()
        .map(|x| {
            if x.len() == 1 {
                x[0].to_string()
            } else {
                format!("{}-{}", x.first().unwrap(), x.last().unwrap())
            }
        })
        .collect::<Vec<String>>()
        .join(",")
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
/// - Return a Vector of path packed
pub fn basic_listing(frames: Paths) -> PathsPacked {
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut frames_list: Vec<String> = frames_dict
        .into_par_iter()
        .map(|(key, value)| {
            if value[0] == "None" && value.len() == 1 {
                key
            } else {
                format!("{}@{}", key, create_frame_string(value))
            }
        })
        .collect();
    frames_list.sort();

    let paths_packed_data = frames_list
        .iter()
        .map(|s| PathBuf::from(s)) // Convert to PathBuf
        .collect::<Vec<PathBuf>>();

    PathsPacked::from(paths_packed_data)
}

/// This function is intented to check if a file is an exr to call exr module
/// and print the exr metadata of the file
fn get_exr_metada(root_path: &String, path: &String) -> String {
    lazy_static! {
        static ref RE_EXR: Regex = Regex::new(r".*.exr$").expect("Can't compile regex");
    }
    if RE_EXR.is_match(&path) {
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
///
/// ### Example of output
/// ```bash
/// ./samples/small/RenderPass_Beauty_1_*****.exr@0-9    1920x1080, RGBA
/// ```
/// ### Example of output with non exr file
/// ```bash
/// ./samples/small/foo.exr    Not an exr
/// ```
pub fn extended_listing(root_path: String, frames: Paths) -> PathsPacked {
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut out_frames: PathsPacked = PathsPacked::new_empty();
    for (key, value) in frames_dict {
        if value[0] == "None" && value.len() == 1 {
            out_frames.push_metadata(get_exr_metada(&root_path, &key));
            out_frames.push_paths(key.into());
        } else {
            let to = value.first().unwrap();
            let from = String::from_utf8(vec![b'*'; to.len()]).unwrap();
            let new_path = &key.replace(&from, to);
            out_frames.push_metadata(get_exr_metada(&root_path, &new_path));
            out_frames.push_paths(format!("{}@{}", key, create_frame_string(value)).into());
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
    let source: &str = "foobar.exr";
    let expected: (String, String) = (source.to_string(), "None".to_string());
    assert_eq!(expected, extract_regex(source))
}

#[test]
fn test_regex_simple() {
    let source: &str = "RenderPass_Beauty_1_00000.exr";
    let expected: (String, String) = (
        "RenderPass_Beauty_1_*****.exr".to_string(),
        "00000".to_string(),
    );
    assert_eq!(expected, extract_regex(source))
}
#[test]
fn test_parse_string() {
    let source: Paths = Paths::from(vec![
        "toto.001.tiff".into(),
        "toto.002.tiff".into(),
        "toto.003.tiff".into(),
        "foo.exr".into(),
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
fn test_continuity() {
    let source: Vec<isize> = vec![1, 2, 3, 5, 6, 7, 11, 12];
    let expected: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12]];
    assert_eq!(expected, group_continuity(&source));
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

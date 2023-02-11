mod exr_metadata;
use crate::exr_metadata::read_meta;
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

pub fn parse_dir(input_path: String) -> Vec<String> {
    let paths: fs::ReadDir = fs::read_dir(input_path).unwrap();
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
    assert_eq!(6, crate::parse_dir("./samples/small".to_string()).len());
}

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
#[test]
fn test_handle_none() {
    let re = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$").unwrap();
    let source: String = "foobar.exr".to_string();
    let expected: (String, String) = (source.clone(), "None".to_string());
    assert_eq!(expected, extract_regex(&re, source))
}

#[test]
fn test_regex_simple() {
    let re = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$").unwrap();
    let source: String = "RenderPass_Beauty_1_00000.exr".to_string();
    let expected: (String, String) = (
        "RenderPass_Beauty_1_*****.exr".to_string(),
        "00000".to_string(),
    );
    assert_eq!(expected, extract_regex(&re, source))
}

fn parse_result(dir_scan: Vec<String>) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$").unwrap();
    let extracted: Vec<(String, String)> = if dir_scan.len() < 100000 {
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
#[test]
fn test_parse_string() {
    let source = vec![
        "toto.001.tiff".to_string(),
        "toto.002.tiff".to_string(),
        "toto.003.tiff".to_string(),
        "foo.exr".to_string(),
    ];
    let vec_toto: Vec<String> = vec!["001".to_string(), "002".to_string(), "003".to_string()];
    let vec_foo: Vec<String> = vec!["None".to_string()];
    let expected: HashMap<String, Vec<String>> = HashMap::from([
        ("toto.***.tiff".to_string(), vec_toto),
        ("foo.exr".to_string(), vec_foo),
    ]);
    assert_eq!(expected, parse_result(source));
}

fn convert_vec(frames_vec: Vec<String>) -> Vec<isize> {
    let mut out_vec: Vec<isize> = frames_vec
        .into_iter()
        .map(|x: String| x.parse::<isize>().unwrap())
        .collect();
    out_vec.sort();
    out_vec
}

#[test]
fn test_convert_vec() {
    let source: Vec<String> = vec!["001".to_string(), "005".to_string(), "003".to_string()];
    let expected: Vec<isize> = vec![1, 3, 5];
    assert_eq!(expected, convert_vec(source));
}

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
#[test]
fn test_continuity() {
    let source: Vec<isize> = vec![1, 2, 3, 5, 6, 7, 11, 12];
    let expected: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12]];
    assert_eq!(expected, group_continuity(&source));
}
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
#[test]
fn test_convert_vec_to_str() {
    let source: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12], vec![45]];
    let expected: String = "1-3,5-7,11-12,45".to_string();
    assert_eq!(expected, convert_vec_to_str(source));
}

pub fn basic(frames: Vec<String>) -> Vec<String> {
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut out_frames: Vec<String> = Vec::new();
    for (key, value) in frames_dict {
        if value[0] == "None" && value.len() == 1 {
            out_frames.push(key);
        } else {
            let i = convert_vec(value);
            let j = group_continuity(&i);
            let k = convert_vec_to_str(j);
            out_frames.push(format!("{}@{}", key, k));
        }
    }
    out_frames
}

pub fn listing(root_path: String, frames: Vec<String>) -> Vec<String> {
    let re: Regex = Regex::new(r".*.exr$").unwrap();
    let frames_dict: HashMap<String, Vec<String>> = parse_result(frames);
    let mut out_frames: Vec<String> = Vec::new();
    for (key, value) in frames_dict {
        if value[0] == "None" && value.len() == 1 {
            out_frames.push(key);
        } else {
            let to = value.first().unwrap();
            let from = String::from_utf8(vec![b'*'; to.len()]).unwrap();
            let new_path = &key.replace(&from, to);
            if re.is_match(new_path) {
                let path = format!("{}{}", root_path, new_path);
                read_meta(path);
            };
            let i = convert_vec(value);
            let j = group_continuity(&i);
            let k = convert_vec_to_str(j);
            out_frames.push(format!("{}@{}", key, k));
        }
    }
    out_frames
}

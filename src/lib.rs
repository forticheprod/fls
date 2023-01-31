use regex::{Captures, Regex};
use std::collections::HashMap;

fn extract_regex(x: String) -> (String, String) {
    let re: Regex = Regex::new(r"(?x)(?P<name>.*)\.(?P<frames>\d{2,9})\.(?P<ext>\w{2,5})").unwrap();
    let result_caps: Option<Captures> = re.captures(&x);
    match result_caps {
        None => (x, "1".to_string()),
        caps_wrap => {
            let caps = caps_wrap.unwrap();
            let string_list = vec![
                caps["name"].to_string(),
                "*".to_string(),
                caps["ext"].to_string(),
            ];
            let key: String = string_list.join(".");
            (key, caps["frames"].to_string())
        }
    }
}
#[test]
fn test_handle_none() {
    let source: String = "foobar.exr".to_string();
    let expected: (String, String) = (source.clone(), "1".to_string());
    assert_eq!(expected, extract_regex(source))
}

pub fn parse_result(dir_scan: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut book_reviews: HashMap<String, Vec<String>> = HashMap::new();
    for x in dir_scan {
        let extraction: (String, String) = extract_regex(x);
        let vec1: Vec<String> = vec![extraction.1.clone()];
        book_reviews
            .entry(extraction.0)
            .and_modify(|value| (*value).push(extraction.1))
            .or_insert(vec1);
    }
    book_reviews
}
#[test]
fn test_parse_string() {
    let source = vec![
        "toto.001.tiff".to_string(),
        "toto.002.tiff".to_string(),
        "foo.exr".to_string(),
    ];
    let vec_toto: Vec<String> = vec!["001".to_string(), "002".to_string()];
    let vec_foo: Vec<String> = vec!["1".to_string()];
    let expected: HashMap<String, Vec<String>> = HashMap::from([
        ("toto.*.tiff".to_string(), vec_toto),
        ("foo.exr".to_string(), vec_foo),
    ]);
    assert_eq!(expected, parse_result(source));
}

fn convert_vec(frames_vec: Vec<String>) -> Vec<isize> {
    frames_vec
        .into_iter()
        .map(|x: String| x.parse::<isize>().unwrap())
        .collect()
}

#[test]
fn test_convert_vec() {
    let source: Vec<String> = vec!["001".to_string(), "003".to_string()];
    let expected: Vec<isize> = vec![1, 3];
    assert_eq!(expected, convert_vec(source));
}

fn group_continuity(data: &[isize]) -> Vec<&[isize]> {
    let mut slice_start: usize = 0;
    let mut result: Vec<&[isize]> = Vec::new();
    for i in 1..data.len() {
        if data[i - 1] + 1 != data[i] {
            result.push(&data[slice_start..i]);
            slice_start = i;
        }
    }
    if data.len() > 0 {
        result.push(&data[slice_start..]);
    }
    result
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
        if x.len() == 0 {
            tmp_vec.push(x[0].to_string());
        } else {
            tmp_vec.push(format!("{}-{}", x.first().unwrap(), x.last().unwrap()))
        }
    }
    tmp_vec.join(",")
}
#[test]
fn test_convert_vec_to_str() {
    let source: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12]];
    let expected: String = "1-3,5-7,11-12".to_string();
    assert_eq!(expected, test_continuity(source));
}

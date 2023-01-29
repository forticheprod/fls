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

fn convert_vec(frames_vec: Vec<String>) -> Vec<i32> {
    frames_vec
        .into_iter()
        .map(|x: String| x.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test_convert_vec() {
    let source: Vec<String> = vec!["001".to_string(), "003".to_string()];
    let expected: Vec<i32> = vec![1, 3];
    assert_eq!(expected, convert_vec(source));
}

fn check_continuity(frames_vec: Vec<String>) -> Vec<String> {
    let mut frames_seq: Vec<String> = Vec::new();
    let tmp_vec: Vec<i32> = convert_vec(frames_vec);
    let last = tmp_vec.last().copied().unwrap();
    let mut continuity: bool = true;
    for (i, frame) in tmp_vec.iter().enumerate() {
        if frame == &tmp_vec[0] || frame == &last {
            //Get the first or the last
            frames_seq.push(frame.to_string());
            continuity = true;
        } else if frame - 1 != tmp_vec[i - 1] {
            //Catch the continuity brake
            frames_seq.push(tmp_vec[i - 1].to_string());
            frames_seq.push(frame.to_string());
            continuity = false;
        }
    }
    frames_seq
}
#[test]
fn test_continuity() {
    let source: Vec<String> = vec![
        "001".to_string(),
        "002".to_string(),
        "003".to_string(),
        "005".to_string(),
        "006".to_string(),
        "007".to_string(),
        "011".to_string(),
        "012".to_string(),
    ];
    let expected: Vec<String> = vec!["1".to_string(), "3".to_string()];
    assert_eq!(expected, check_continuity(source));
}

fn convert_frames(frames_vec: Vec<String>) -> String {
    "toto".to_string()
}

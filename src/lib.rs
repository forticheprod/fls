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

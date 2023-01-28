use regex::{Captures, Regex};
use std::{collections::HashMap, usize};

fn extract_regex(x: String) -> (String, String) {
    let re: Regex = Regex::new(r"(?x)(?P<name>.*)\.(?P<frames>\d{2,9})\.(?P<ext>\w{2,5})").unwrap();
    let result_caps = re.captures(&x).unwrap_unchecked();
    match result_caps {
        caps => {
            let string_list = vec![
                caps["name"].to_string(),
                "*".to_string(),
                caps["ext"].to_string(),
            ];
            let key: String = string_list.join(".");
            (key, caps["frames"].to_string())
        }
        None => (x, "1".to_string()),
        _ => panic!("Nothing found"),
    }
}

fn parse_result(dir_scan: Vec<String>) -> HashMap<String, String> {
    let mut book_reviews: HashMap<String, String> = HashMap::new();
    for x in dir_scan {
        let extraction: (String, String) = extract_regex(x);
        book_reviews
            .entry(extraction.0)
            .and_modify(|value: &mut String| (*value).push_str(&extraction.1))
            .or_insert(extraction.1);
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
    let expected = HashMap::from([("toto.*.tiff".to_string(), "001002".to_string())]);
    assert_eq!(expected, parse_result(source));
}

use regex::Regex;
use std::collections::HashMap;

fn parse_result(dir_scan: Vec<String>) -> HashMap<String, String> {
    let mut book_reviews = HashMap::new();
    let re: Regex = Regex::new(r"(?x)(?P<name>.*)\.(?P<frames>\d{2,9})\.(?P<ext>\w{2,5})").unwrap();
    for x in dir_scan {
        let caps = re.captures(&x).unwrap();
        let string_list = vec![
            caps["name"].to_string(),
            "*".to_string(),
            caps["ext"].to_string(),
        ];
        let key: String = string_list.join(".");
        let mut value_cont = ";".to_string();
        value_cont.push_str(&caps["frames"]);
        book_reviews
            .entry(key)
            .and_modify(|value: &mut String| (*value).push_str(&value_cont))
            .or_insert(caps["frames"].to_string());
    }
    book_reviews
}
#[test]
fn test_parse_string() {
    let source = vec!["toto.001.tiff".to_string(), "toto.002.tiff".to_string()];
    let expected = HashMap::from([("toto.*.tiff".to_string(), "001;002".to_string())]);
    assert_eq!(expected, parse_result(source));
}

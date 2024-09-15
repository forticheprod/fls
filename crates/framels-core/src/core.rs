use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::PathBuf;

/// This function extract the matching group based on regex already compile to
/// a tuple of string. For exemple toto.458.jpg should return
/// (toto.***.jpg, 458)
#[inline(always)]
fn extract_regex(x: &str) -> (String, String) {
    lazy_static! {
        static ref RE_FLS: Regex = Regex::new(r"(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$")
            .expect("Can't compile regex");
    }
    let result_caps: Option<Captures> = RE_FLS.captures(x);
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
pub(crate) fn parse_result(
    list_paths: Vec<PathBuf>,
    multithreaded: bool,
) -> HashMap<String, Vec<String>> {
    // Optimisation over PAR_THRESHOLD value, the parsing of the frame list
    // used rayon lib to paralelize the work. Result depends a lot from the
    // cpu number of thread may be put in a config file
    const PAR_THRESHOLD: usize = 100000;
    let extracted: Vec<(String, String)> = if (list_paths.len() > PAR_THRESHOLD) | multithreaded {
        list_paths
            .par_iter()
            .map(|path| extract_regex(path.to_str().unwrap()))
            .collect()
    } else {
        list_paths
            .iter()
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
pub(crate) fn create_frame_string(value: Vec<String>) -> String {
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

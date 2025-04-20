//! # framels
//!
//! **framels** is a library and a binary to list files and directorys like `ls`
//! style and return a packed
//! sequence of files. This lib is industry oriented for Animation and VFX,
//! using a lot a frames sequences.
//! The main objective is to be the fastest as possible using rustlang.
//!
//! ## Key concept
//!
//! The key concept of the library is to pack frames sequences in a new
//! filename like `toto.***.jpg@158-179`. It use a regex to extract the frame
//! number and pack the frames. The regex is `(?x)(?P<name>.*)(?P<sep>\.|_)(?P<frames>\d{2,9})\.(?P<ext>\w{2,5})$`.
//! It results in the limitations of only:
//!
//! - `.` or `_` as a separator between the filename and the frame number.
//! - 2 to 9 digits for the frame number.
//! - 2 to 5 characters for the extension.
//! - The frame number must be at the end of the filename.
//! - The frame number must be a number.
//! - The frame number must be a positive number.
//! - There is not filetering in the extension.
//!
//!
//! ## Command line
//!
//!
//! framels is also a command line tool to list directory and pack frames in
//! sequences. It's a bit like `ls` but for frames sequences.
//! You can use the short cut `fls` to call the binary.
//!
//! ## Example
//!
//! In this example we use the recursive option to list all the files in the
//! directory and subdirectory.
//!
//! ```bash
//! $ fls --recursive ./samples/small
//! ./samples/small
//! ./samples/small/aaa.***.tif@1-5
//! ./samples/small/foo_bar.ex
//! ```
//!
//! ## Library
//!
//! The library is the core of the binary, it's a rust library to list
//! directory and pack frames in sequences.
//!
//! ### listing content of directories
//!
//! #### parse_dir
//!
//! The [parse_dir] function list files and directories in the targeted
//! directory, take a `String` as input and return a [Paths] of the entries.
//! It use [std::fs::ReadDir], nothing much.
//!
//! #### recursive_dir
//!
//! The [recursive_dir] function list files and directories in the targeted
//! directory, take a `String` as inut and return a [Paths] of the
//! entries recursively. It use [jwalk] as the core of the parsing. It is really
//! efficiant with a lot of directories.
//!
//! ### pack frames
//!
//! The library is really simple to use, you can use the [basic_listing] or
//! [extended_listing] function to list directory and pack frames in sequences.
//!
//! The frame packing algorithm got a optimization when the amount of listed
//! files is bigger than 100 000 files.
//!
//! #### basic_listing
//!
//! The [basic_listing] function is the main function of the library it use a
//! list of filename as in input and pack the frame sequences using a new
//! filename like `toto.***.jpg@158-179`
//!
//! #### extended_listing
//!
//! The [extended_listing] function is specialize to analyse exr frames really
//! similar to `rvls -l`
//! It take a `Vec<String>` of entries as an input
//! - Pack the frames
//! - Print the metadata if the sequence is an exr sequence
//! - Return a Vector of path packed
//!
//! ### Formating the output
//! 
//! The output can be formated using the [FormatTemplate] struct.
//!  
//! Three formats are available, with `toto.160.jpg` as an example:
//!  - default : `{name}{sep}{padding}.{ext}@{first_frame}-{last_frame}` => `toto.***.jpg@158-179`
//!  - buf : `{name}{sep}[{first_frame}:{last_frame}].{ext}` => `toto.[158:179].jpg`
//!  - nuke : `{name}{sep}.{ext} {first_frame}-{last_frame}`=> `toto.jpg 158-179`
//! 
//! 
//! ### Example
//!
//! ```rust
//! use framels::{basic_listing, extended_listing, parse_dir, paths::{Paths,Join}, recursive_dir, FormatTemplate};
//!
//! fn main() {
//!    // Perform directory listing
//!   let in_paths: Paths = parse_dir("./samples/small");
//!
//!  // Generate results based on arguments
//! let results: String = basic_listing(in_paths, false,
//!     FormatTemplate::default().format).get_paths().join("\n");
//!
//! println!("{}", results)
//! }
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
use strfmt::strfmt;

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
/// a tuple of (file_padded:String, frame_number:String, hashmap<re_group:String, capture:String>)
/// For example toto.459.jpg should return:
/// (toto.***.jpg, 458, [("name", "toto"), ("sep", "."), ("frames", "458"), ("ext", "jpg")])
#[inline(always)]
fn extract_regex(x: &str) -> (String, Option<HashMap<String, String>>) {
    lazy_static! {
        static ref RE_FLS: Regex =
            Regex::new(r"(?x)(?P<name>.*)(?P<sep>\.|_)(?P<frames>\d{2,9})\.(?P<ext>\w{2,5})$")
                .expect("Can't compile regex");
    }
    let result_caps: Option<Captures> = RE_FLS.captures(&x);
    match result_caps {
        None => (x.to_string(), None),
        Some(caps) => {
            let hashmap: HashMap<String, String> = RE_FLS
                .capture_names()
                .flatten()
                .filter_map(|n| {
                    caps.name(n)
                        .map(|m| (n.to_string(), m.as_str().to_string()))
                })
                .collect();
            (
                x.replace(&caps["frames"], &"*".repeat(caps["frames"].len())),
                Some(hashmap),
            )
        }
    }
}

/// A struct representing a collection of frames with associated information.
///
/// # Fields
///
/// * `name` - The name of the frame collection.
/// * `sep` - A separator string used in the frame collection.
/// * `frames` - A vector containing the frame data as integers.
/// * `ext` - The file extension associated with the frames.
/// * `padding` - The amount of padding to be applied to the frames.
#[derive(Debug)]
struct Frames {
    name: String,
    sep: String,
    frames: Vec<i32>,
    ext: String,
    padding: usize,
}

impl Frames {
    /// Creates a `Frames` struct from a `HashMap`.
    ///
    /// # Parameters
    ///
    /// * `_hashmap` - A reference to a `HashMap<String, String>` containing the frame details.
    /// name, sep, frames, ext, padding
    /// # Returns
    ///
    /// A `Frames` struct populated with the values from the `HashMap`.
    fn from_hashmap(_hashmap: &HashMap<String, String>) -> Self {
        let name = _hashmap.get("name").unwrap_or(&"None".to_string()).clone();
        let sep = _hashmap.get("sep").unwrap_or(&"None".to_string()).clone();
        let frame = _hashmap
            .get("frames")
            .unwrap_or(&"None".to_string())
            .clone();
        let frames = vec![frame.parse().unwrap_or(0)];
        let ext = _hashmap.get("ext").unwrap_or(&"None".to_string()).clone();
        let padding = frame.len();
        
        Frames {
            name,
            sep,
            frames,
            ext,
            padding,
        }
    }
    /// Returns the minimum value in `self.frames`.
    ///
    /// # Returns
    ///
    /// An `Option<i32>` containing the minimum value in `self.frames`, or `None` if `self.frames` is empty.
    fn first_frame(&self) -> Option<i32> {
        self.frames.iter().min().copied()
    }
    /// Returns the maximum value in `self.frames`.
    ///
    /// # Returns
    ///
    /// An `Option<i32>` containing the maximum value in `self.frames`, or `None` if `self.frames` is empty.
    fn last_frame(&self) -> Option<i32> {
        self.frames.iter().max().copied()
    }
    /// Returns a vector of strings representing the frame list. Used to print the missing frames.
    /// 
    /// # Returns
    /// 
    /// A `Vec<String>` containing the frame list as a string.
    fn frame_list(&self) -> Vec<String>{
        let frames_as_isize: Vec<isize> = self.frames.iter()
            .map(|frame| *frame as isize).collect();
            group_continuity(&frames_as_isize).iter()
            .filter(|ve| !ve.is_empty())
            .map(|ve| format!("{}:{}", ve[0], ve[ve.len()-1]))
            .collect()

    }
    /// Converts the `Frames` struct to a `HashMap` of string values.
    ///
    /// This was implemented to be used with `strfmt::strfmt`
    ///
    /// # Returns
    ///
    /// A `HashMap<String, String>` containing the following key-value pairs:
    ///
    /// * `"name"` - The name of the frame.
    /// * `"sep"` - The separator used in the frame.
    /// * `"ext"` - The extension of the frame.
    /// * `"first_frame"` - The first frame number as a string, or `"None"` if `self.frames` is empty.
    /// * `"last_frame"` - The last frame number as a string, or `"None"` if `self.frames` is empty.
    /// * `"padding"` - A string of asterisks (`'*'`) representing the padding length.
    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), self.name.clone());
        map.insert("sep".to_string(), self.sep.clone());
        map.insert("ext".to_string(), self.ext.clone());
        map.insert(
            "first_frame".to_string(),
            self.first_frame()
                .map_or("None".to_string(), |f| f.to_string()),
        );
        map.insert(
            "last_frame".to_string(),
            self.last_frame()
                .map_or("None".to_string(), |f| f.to_string()),
        );
        map.insert(
            "padding".to_string(),
            std::iter::repeat('*').take(self.padding.into()).collect(),
        );
        map.insert(
            "frame_list".to_string(),
            self.frame_list().join(","),
        );
        map
    }
}

impl PartialEq for Frames {
    /// Add a PartialEq implementation to the Frames struct
    /// mainly for testing purpose.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.sep == other.sep
            && self.frames == other.frames
            && self.ext == other.ext
            && self.padding == other.padding
    }
}

/// Parse the result of a vector of string. This function use HashMap to pack
/// filename removed from the frame value.
fn parse_result(dir_scan: Paths, multithreaded: bool) -> HashMap<String, Option<Frames>> {
    // Optimisation over PAR_THRESHOLD value, the parsing of the frame list
    // used rayon lib to paralelize the work. Result depends a lot from the
    // cpu number of thread may be put in a config file
    const PAR_THRESHOLD: usize = 100000;
    let extracted: Vec<(String, Option<HashMap<String, String>>)> =
        if (dir_scan.len() > PAR_THRESHOLD) | multithreaded {
            dir_scan
                .par_iter()
                .map(|path| extract_regex(path.to_str().unwrap()))
                .collect()
        } else {
            dir_scan
                .iter()
                .map(|path| extract_regex(path.to_str().unwrap()))
                .collect()
        };

    let mut paths_dict: HashMap<String, Option<Frames>> = HashMap::with_capacity(extracted.len());
    for extraction in extracted {
        paths_dict
            .entry(extraction.0)
            .and_modify(|f| match f {
                None => (),
                Some(f) => match &extraction.1 {
                    None => (),
                    Some(t) => f.frames.push(t.get("frames").unwrap().parse().unwrap_or(0)),
                },
            })
            .or_insert(match &extraction.1 {
                None => None,
                Some(t) => Some(Frames::from_hashmap(t)),
            });
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

/// Structure to store output string format templates used in the library.
/// These templates are strings that can be used with the [strfmt()] crate to
/// model the output.
pub struct FormatTemplate {
    pub format: &'static str,
}

impl Default for FormatTemplate {
    /// Default format template
    /// 
    /// `toto.160.jpg` => `toto.***.jpg@158-179`
    fn default() -> Self{
        FormatTemplate {
            format: "{name}{sep}{padding}.{ext}@{first_frame}-{last_frame}"
        }
    }
}

impl FormatTemplate {
    /// Buff format template
    /// 
    /// `toto.160.jpg` => `toto.[158:179].jpg`
    pub fn buf_format () -> Self {
        FormatTemplate {
            format: "{name}{sep}[{frame_list}].{ext}"
        }
    }
    /// Nuke format template
    /// 
    /// `toto.160.jpg` => `toto.jpg 158-179`
    pub fn nuke_format() -> Self {
        FormatTemplate {
            format: "{name}{sep}.{ext} {first_frame}-{last_frame}"
        }
    }
}

/// # basic_listing
///
///
/// ## Description
///
/// This function is the main function of the library it use a list of
/// filename as in input and pack the frame sequences using a new filename
/// like `toto.***.jpg@158-179`
///
/// It take a `Vec<String>` of entries as an input
///  - Pack the frames
/// - Return a Vector of path packed
///
/// ## Example
///
/// ### Example of output
///
/// ```bash
/// ./samples/small/aaa.***.tif@1-5
/// ./samples/small/foo_bar.ex
/// ```
///
/// ### Example of output with non exr file
///
/// ```bash
/// ./samples/small/foo.exr@None
/// ```
///
/// ### Example as a library
///
/// ```rust
/// use framels::{basic_listing, parse_dir, paths::{Paths,Join}, FormatTemplate};
///
/// fn main() {
///     // Perform directory listing
///     let in_paths: Paths = parse_dir("./samples/small");
///
///     // Generate results based on arguments
///     let results: String = basic_listing(in_paths, false, FormatTemplate::default().format).get_paths().join("\n");
///
///      println!("{}", results)
/// }
/// ```
pub fn basic_listing(frames: Paths, multithreaded: bool, format: &str) -> PathsPacked {
    let frames_dict = parse_result(frames, multithreaded);
    let mut frames_list: Vec<String> = frames_dict
        .into_par_iter()
        .map(|(key, value)| {
            match value {
                None => key,
                Some(f) => match strfmt(format, &f.to_hashmap()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprint!("Error formatting string: {}", e);
                        String::new()
                    }
                },
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

/// # extended_listing
///
///
/// ## Description
///
/// This function is specialize to analyse exr frames really similar to
/// `rvls -l`
///
/// It take a `Vec<String>` of entries as an input
///  - Pack the frames
///  - Print the metadata if the sequence is an exr sequence
///  - Return a Vector of path packed
///
/// ## Example
///
/// ### Example of output
/// ```bash
/// ./samples/small/RenderPass_Beauty_1_*****.exr@0-9    1920x1080, RGBA
/// ```
/// ### Example of output with non exr file
/// ```bash
/// ./samples/small/foo.exr    Not an exr
/// ```
pub fn extended_listing(root_path: String, frames: Paths, multithreaded: bool, format: &str) -> PathsPacked {
    let frames_dict = parse_result(frames, multithreaded);
    let mut out_frames: PathsPacked = PathsPacked::new_empty();
    for (key, value) in frames_dict {
        match value {
            None => {
                out_frames.push_metadata(get_exr_metada(&root_path, &key));
                out_frames.push_paths(key.into())
            }
            Some(f) => {
                let to = f.frames.first().unwrap();
                let frame = format!("{:0zfill$}", to, zfill = f.padding);
                let new_path = format!(
                    "{name}{sep}{frame}.{ext}",
                    name = f.name,
                    sep = f.sep,
                    ext = f.ext
                );
                out_frames.push_metadata(get_exr_metada(&root_path, &new_path));
                out_frames.push_paths(PathBuf::from(
                    strfmt(&format, &f.to_hashmap()).unwrap(),
                ));
            }
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
    let expected: (String, Option<Frames>) = (source.to_string(), None);
    let extraction = extract_regex(source);
    assert_eq!(expected.0, extraction.0);
    assert!(extraction.1.is_none())
}

#[test]
fn test_regex_simple() {
    let source: &str = "RenderPass_Beauty_1_00000.exr";
    let expected: (String, Option<HashMap<String, String>>) = (
        "RenderPass_Beauty_1_*****.exr".to_string(),
        Some(HashMap::from([
            ("name".to_string(), "RenderPass_Beauty_1".to_string()),
            ("sep".to_string(), "_".to_string()),
            ("frames".to_string(), "00000".to_string()),
            ("ext".to_string(), "exr".to_string()),
        ])),
    );
    let extraction = extract_regex(source);
    assert_eq!(expected.0, extraction.0);
    assert!(extraction.1.is_some());
    assert_eq!(expected.1, extraction.1)
}
#[test]
fn test_parse_string() {
    let source: Paths = Paths::from(vec![
        "toto.001.tiff".into(),
        "toto.002.tiff".into(),
        "toto.003.tiff".into(),
        "foo.exr".into(),
    ]);
    let frames_toto = Frames {
        name: "toto".to_string(),
        sep: ".".to_string(),
        frames: vec![1, 2, 3],
        ext: "tiff".to_string(),
        padding: 3,
    };
    let expected: HashMap<String, Option<Frames>> = HashMap::from([
        ("toto.***.tiff".to_string(), Some(frames_toto)),
        ("foo.exr".to_string(), None),
    ]);
    assert_eq!(expected, parse_result(source, false));
}
#[test]
fn test_continuity() {
    let source: Vec<isize> = vec![1, 2, 3, 5, 6, 7, 11, 12];
    let expected: Vec<Vec<isize>> = vec![vec![1, 2, 3], vec![5, 6, 7], vec![11, 12]];
    assert_eq!(expected, group_continuity(&source));
}

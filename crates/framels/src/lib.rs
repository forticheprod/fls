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
//! number and pack the frames. The regex is `(?x)(.*)(\.|_)(?P<frames>\d{2,9})\.(\w{2,5})$`.
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
//! - Print the metada if the sequence is an exr sequence
//! - Return a Vector of path packed
//!
//! ### Example
//!
//! ```rust
//! use framels::{extended_listing, parse_dir, recursive_dir};
//! use framels_core::{PathsPacked,Paths};
//!
//! // Perform directory listing
//! let in_paths: Paths = parse_dir("./samples/small");
//!
//! // Generate results based on arguments
//! let results: String = in_paths.pack(false).join("\n");
//!
//! println!("{}", results)
//! ```
mod exr_metadata;
use crate::exr_metadata::read_meta;
use framels_core::{Paths, PathsPacked};
use jwalk::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

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

/// This function is intented to check if a file is an exr to call exr module
/// and print the exr metadata of the file
fn get_exr_metada(root_path: &String, path: &String) -> String {
    lazy_static! {
        static ref RE_EXR: Regex = Regex::new(r".*.exr$").expect("Can't compile regex");
    }
    if RE_EXR.is_match(path) {
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
///  - Print the metada if the sequence is an exr sequence
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
pub fn extended_listing(root_path: String, frames: Paths, multithreaded: bool) -> PathsPackedMeta {
    let paths_packed: PathsPacked = frames.pack(multithreaded);
    let paths: Vec<PathBuf> = paths_packed.get_paths();
    let metadata: Vec<String> = paths
        .iter()
        .map(|path| get_exr_metada(&root_path, &path.to_str().unwrap().to_string()))
        .collect();
    PathsPackedMeta {
        paths: paths_packed,
        metadata,
    }
}

pub struct PathsPackedMeta {
    paths: PathsPacked,
    metadata: Vec<String>,
}

impl PathsPackedMeta {
    pub fn new_empty() -> Self {
        PathsPackedMeta {
            paths: PathsPacked::new_empty(),
            metadata: Vec::new(),
        }
    }
    pub fn from(paths: PathsPacked, metadata: Vec<String>) -> Self {
        PathsPackedMeta { paths, metadata }
    }

    pub fn push_paths(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn push_metadata(&mut self, meta: String) {
        self.metadata.push(meta);
    }

    pub fn get_paths(&self) -> Vec<PathBuf> {
        self.paths.get_paths()
    }

    pub fn get_metadata(&self) -> Vec<String> {
        self.metadata.clone()
    }
    /// Join the paths and the metadata
    pub fn join(&self, sep: &str) -> String {
        let paths_strings: Vec<String> = self
            .paths
            .get_paths()
            .iter()
            .map(|f| f.to_string_lossy().into_owned())
            .collect();

        let main_vec = paths_strings
            .into_iter()
            .chain(self.metadata.iter().cloned());
        main_vec.collect::<Vec<String>>().join(sep)
    }
}

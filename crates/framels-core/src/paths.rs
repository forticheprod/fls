//! paths module is handle the Paths and PathsPacked types

use crate::core::{create_frame_string, parse_result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::{clone::Clone, path::PathBuf};
/// # Paths
///
/// ## Description
///
/// The Paths struct is a representation of a group of paths.
///
/// It contains a Vec of PathBuf.
/// It is used to store the paths of the files listed by the listing functions.
/// It also used to store the paths of the files to be processed by the processing functions.
///
/// ## Example
///
/// ```rust
/// use framels::paths::Paths;
/// use std::path::PathBuf;
///
/// let paths = Paths::from(vec![PathBuf::from("foo"), PathBuf::from("bar")]);
/// assert_eq!(paths.len(), 2);
/// ```
#[derive(Clone)]
pub struct Paths {
    data: Vec<PathBuf>, // Use PathBuf instead of String
}

impl Paths {
    /// # len
    ///
    /// ## Description
    ///
    /// Return the number of paths
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::Paths;
    /// use std::path::PathBuf;
    ///
    /// let paths = Paths::from(vec![PathBuf::from("foo"), PathBuf::from("bar")]);
    /// assert_eq!(paths.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// # from
    ///
    /// ## Description
    ///
    /// Create a [Paths] from a [Vec] of [PathBuf]
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::Paths;
    /// use std::path::PathBuf;
    ///
    /// let paths = Paths::from(vec![PathBuf::from("foo"), PathBuf::from("bar")]);
    /// assert_eq!(paths.len(), 2);
    /// ```
    pub fn from(data: Vec<PathBuf>) -> Self {
        Paths { data }
    }
    /// # to_vec
    ///
    /// ## Description
    ///
    /// Create a [Vec] of [PathBuf] from a [Paths]
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::Paths;
    /// use std::path::PathBuf;
    ///
    /// let input_vec = vec![PathBuf::from("foo"), PathBuf::from("bar")];
    /// let paths = Paths::from(input_vec.clone());
    /// let output_vec = paths.to_vec();
    /// assert_eq!(input_vec, output_vec);
    /// ```
    pub fn to_vec(&self) -> Vec<PathBuf> {
        self.data.clone()
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
    /// use framels_core::Paths;
    /// use std::path::PathBuf;
    ///
    /// // Perform directory listing
    /// let in_paths: Paths = Paths::from(vec![
    /// PathBuf::from("./samples/small/aaa.001.tif"),
    /// PathBuf::from("./samples/small/aaa.002.tif"),
    /// PathBuf::from("./samples/small/aaa.003.tif"),
    /// PathBuf::from("./samples/small/aaa.004.tif"),
    /// PathBuf::from("./samples/small/aaa.005.tif"),
    /// PathBuf::from("./samples/small/foo_bar.ex"),
    /// ]);
    ///
    /// // Generate results based on arguments
    /// let results: String = in_paths.pack(false).get_paths().join("\n");
    ///
    /// println!("{}", results)
    /// ```
    pub fn pack(self, multithreaded: bool) -> PathsPacked {
        let frames_dict: HashMap<String, Vec<String>> = parse_result(self.data, multithreaded);
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
            .map(PathBuf::from) // Convert to PathBuf
            .collect::<Vec<PathBuf>>();

        PathsPacked::from(paths_packed_data)
    }
    pub fn join(&self, sep: &str) -> String {
        self.data
            .iter()
            .map(|f| f.to_string_lossy())
            .collect::<Vec<_>>()
            .join(sep)
    }
    pub fn new_empty() -> Self {
        Paths { data: Vec::new() }
    }
    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.data
    }
}

/// A representation of the paths packed based on the Paths struct
/// It contains the paths and the metadata
/// The metadata is a Vec of String
pub struct PathsPacked {
    paths: Paths,
}

impl PathsPacked {
    /// # from_vec
    ///
    /// ## Description
    ///
    /// Create a [PathsPacked] from a [Vec] of [PathBuf]
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::PathsPacked;
    /// use std::path::PathBuf;
    ///
    /// let input_vec = vec![PathBuf::from("foo"), PathBuf::from("bar")];
    /// let paths = PathsPacked::from(input_vec.clone());
    /// let output_vec = paths.get_paths().to_vec();
    /// assert_eq!(input_vec, output_vec);
    pub(crate) fn from(data: Vec<PathBuf>) -> Self {
        PathsPacked {
            paths: Paths::from(data),
        }
    }
    pub fn push(&mut self, path: PathBuf) {
        self.paths.data.push(path);
    }
    pub fn new_empty() -> Self {
        PathsPacked {
            paths: Paths::new_empty(),
        }
    }
    pub fn get_paths(&self) -> Vec<PathBuf> {
        self.paths.to_vec()
    }
    pub fn join(&self, sep: &str) -> String {
        self.paths.join(sep)
    }
}

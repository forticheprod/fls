//! # Paths
//!
//! The Paths struct is a representation of a group of paths.
//!
//! It contains a Vec of PathBuf.
//!
//! It is used to store the paths of the files in a directory.
//!

use rayon::prelude::*;
use std::{clone::Clone, path::PathBuf};

/// # Join
///
/// # Description
///
/// Trait to join the paths of a Paths struct
///
/// # Example
///
/// ```
/// use framels::paths::{Join, Paths};
/// use std::path::PathBuf;
///
/// let paths = Paths::from(vec![PathBuf::from("foo"), PathBuf::from("bar")]);
/// assert_eq!(paths.join(" "), "foo bar");
/// ```
pub trait Join {
    fn join(&self, sep: &str) -> String;
}

pub trait New {
    fn new_empty() -> Self;
}
/// A representation of group of paths
#[derive(Clone)]
pub struct Paths {
    data: Vec<PathBuf>, // Use PathBuf instead of String
}

impl Paths {
    /// Get the number of path
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Iter over the paths
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.data.iter()
    }
    /// Multithread iter over the paths
    pub fn par_iter(&self) -> impl ParallelIterator<Item = &PathBuf> {
        self.data.par_iter()
    }
    /// Create a new Paths from a Vec of PathBuf
    pub fn from(data: Vec<PathBuf>) -> Self {
        Paths { data }
    }
    /// Create a Vector of paths from a Paths
    pub fn to_vec(&self) -> Vec<PathBuf> {
        self.data.clone()
    }
    /// Create a Vecor of PathBuf from a Paths
    pub fn to_vec_path(&self) -> Vec<PathBuf> {
        self.data.iter().cloned().collect()
    }
}

impl Join for Paths {
    fn join(&self, sep: &str) -> String {
        self.data
            .iter()
            .map(|f| f.to_string_lossy())
            .collect::<Vec<_>>()
            .join(sep)
    }
}

impl New for Paths {
    fn new_empty() -> Self {
        Paths { data: Vec::new() }
    }
}

/// A representation of the paths packed based on the Paths struct
/// It contains the paths and the metadata
/// The metadata is a Vec of String
pub struct PathsPacked {
    paths: Paths,
    metadata: Vec<String>,
}

impl PathsPacked {
    pub fn from_vec(data: Vec<PathBuf>) -> Self {
        PathsPacked {
            paths: Paths::from(data),
            metadata: Vec::new(),
        }
    }
    /// Push a path packed
    pub fn push_paths(&mut self, path: PathBuf) {
        self.paths.data.push(path)
    }
    /// Sort the paths attributes
    pub fn sort_paths(&mut self) {
        self.paths.data.sort()
    }
    /// Push a metadata String
    pub fn push_metadata(&mut self, path: String) {
        self.metadata.push(path)
    }
    /// Return a clone of the paths elements
    pub fn get_paths(&self) -> Paths {
        self.paths.clone()
    }
    /// Return a clone of the metadata elements
    pub fn get_metadata(&self) -> Vec<String> {
        self.metadata.clone()
    }
}

impl Join for PathsPacked {
    /// Join the paths and the metadata
    fn join(&self, sep: &str) -> String {
        let paths_strings: Vec<String> = self
            .paths
            .data
            .iter()
            .map(|f| f.to_string_lossy().into_owned())
            .collect();

        let main_vec = paths_strings
            .into_iter()
            .chain(self.metadata.iter().cloned());
        main_vec.collect::<Vec<String>>().join(sep)
    }
}

impl New for PathsPacked {
    fn new_empty() -> Self {
        PathsPacked {
            paths: Paths::new_empty(),
            metadata: Vec::new(),
        }
    }
}

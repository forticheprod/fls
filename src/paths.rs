use rayon::prelude::*;
use std::{clone::Clone, path::PathBuf};

/// A representation of group of paths
#[derive(Clone)]
pub struct Paths {
    data: Vec<String>,
}

impl Paths {
    /// Get the number of path
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Iter over the paths
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }
    /// Multithread iter over the paths
    pub fn par_iter(&self) -> impl ParallelIterator<Item = &String> {
        self.data.par_iter()
    }
    /// Create a new Paths from a Vec of Strings
    pub fn new(data: Vec<String>) -> Self {
        Paths { data }
    }
    /// Create an empty Paths
    fn new_empty() -> Self {
        Paths { data: Vec::new() }
    }
    /// Create a Vector of paths from a Paths
    pub fn to_vec(&self) -> Vec<String> {
        self.data.clone()
    }
    /// Create a Vecor of PathBuf from a Paths
    pub fn to_vec_path(&self) -> Vec<PathBuf> {
        self.data.iter().map(|f| PathBuf::from(f)).collect()
    }
    pub fn join(&self, sep: &str) -> String {
        self.data.join(sep)
    }
}

/// A representation of the paths packed based on the Paths struct
pub struct PathsPacked {
    paths: Paths,
    metadata: Paths,
}

impl PathsPacked {
    /// Create a new PathsPacked empty
    pub fn new_empty() -> Self {
        PathsPacked {
            paths: Paths::new_empty(),
            metadata: Paths::new_empty(),
        }
    }
    pub fn new_from_vec(data: Vec<String>) -> Self {
        PathsPacked {
            paths: Paths::new(data),
            metadata: Paths::new_empty(),
        }
    }
    /// Push a path packed
    pub fn push_paths(&mut self, path: String) {
        self.paths.data.push(path)
    }
    /// Sort the paths attributes
    pub fn sort_paths(&mut self) {
        self.paths.data.sort()
    }
    /// Push a metadata String
    pub fn push_metadata(&mut self, path: String) {
        self.metadata.data.push(path)
    }
    /// Join the paths and the metadata
    pub fn join(&self, sep: &str) -> String {
        let mut main_vec: Vec<String> = self.paths.data.clone();
        main_vec.extend(self.metadata.data.clone());
        main_vec.join(sep)
    }
    /// Return a clone of the paths elements
    pub fn get_paths(&self) -> Paths {
        self.paths.clone()
    }
    /// Return a clone of the metadata elements
    pub fn get_metadata(&self) -> Paths {
        self.metadata.clone()
    }
}

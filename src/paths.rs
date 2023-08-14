use rayon::prelude::*;
use std::{clone::Clone, path::PathBuf};

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
    pub fn new(data: Vec<PathBuf>) -> Self {
        Paths { data }
    }
    /// Create an empty Paths
    fn new_empty() -> Self {
        Paths { data: Vec::new() }
    }
    /// Create a Vector of paths from a Paths
    pub fn to_vec(&self) -> Vec<PathBuf> {
        self.data.clone()
    }
    /// Create a Vecor of PathBuf from a Paths
    pub fn to_vec_path(&self) -> Vec<PathBuf> {
        self.data.iter().cloned().collect()
    }
    pub fn join(&self, sep: &str) -> String {
        self.data.iter().map(|f| f.to_string_lossy()).collect::<Vec<_>>().join(sep)
    }
}

/// A representation of the paths packed based on the Paths struct
pub struct PathsPacked {
    paths: Paths,
    metadata: Vec<String>,
}

impl PathsPacked {
    /// Create a new PathsPacked empty
    pub fn new_empty() -> Self {
        PathsPacked {
            paths: Paths::new_empty(),
            metadata: Vec::new(),
        }
    }
    pub fn new_from_vec(data: Vec<PathBuf>) -> Self {
        PathsPacked {
            paths: Paths::new(data),
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
    /// Join the paths and the metadata
    pub fn join(&self, sep: &str) -> String {
        let paths_strings: Vec<String> = self.paths.data.iter()
            .map(|f| f.to_string_lossy().into_owned())
            .collect();
        
        let main_vec = paths_strings.into_iter().chain(self.metadata.iter().cloned());
        main_vec.collect::<Vec<String>>().join(sep)
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

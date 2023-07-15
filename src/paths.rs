use core;
use rayon::prelude::*;
use std::clone::Clone;

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
    pub fn iter(&self) -> core::slice::Iter<'_, String> {
        self.data.iter()
    }
    /// Multithread iter over the paths
    pub fn par_iter(&self) -> rayon::slice::Iter<'_, String> {
        self.data.par_iter()
    }
    /// Create a new Paths from a Vec of Strings
    pub fn new(data: Vec<String>) -> Paths {
        Paths { data }
    }
    /// Create an empty Paths
    fn new_empty() -> Paths {
        Paths { data: Vec::new() }
    }
    /// Create a Vector of paths from a Paths
    pub fn to_vec(&self) -> Vec<String> {
        self.data.clone()
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
    pub fn new_empty() -> PathsPacked {
        PathsPacked {
            paths: Paths::new_empty(),
            metadata: Paths::new_empty(),
        }
    }
    /// Push a path packed
    pub fn push_paths(&mut self, path: String) {
        self.paths.data.push(path)
    }
    /// Push a metadata String
    pub fn push_metadata(&mut self, path: String) {
        self.metadata.data.push(path)
    }
    /// Join the paths and the metadata
    pub fn join(&self, sep: &str) -> String {
        let mut main_vec = self.paths.data.clone();
        main_vec.extend(self.metadata.data.clone());
        main_vec.join(sep)
    }
    pub fn get_paths(&self)->Paths{
        self.paths.clone()
    }
}

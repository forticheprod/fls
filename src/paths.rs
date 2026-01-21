//! paths module is handle the Paths and PathsPacked types

use rayon::prelude::*;
use std::ops::Deref;
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
    /// Join the list of elements
    fn join(&self, sep: &str) -> String;
}
/// # New
///
/// # Description
///
/// Trait to create an empty Paths or PathsPacked struct
///
pub trait New {
    /// Create emptry struct to init
    fn new_empty() -> Self;
}
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
    /// # is_empty
    ///
    /// ## Description
    ///
    /// Return true if the paths collection is empty, false otherwise
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::Paths;
    /// use std::path::PathBuf;
    ///
    /// let empty_paths = Paths::from(vec![]);
    /// assert_eq!(empty_paths.is_empty(), true);
    ///
    /// let paths = Paths::from(vec![PathBuf::from("foo")]);
    /// assert_eq!(paths.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// # iter
    ///
    /// ## Description
    ///
    /// Return an iterator over the paths
    ///
    /// ## Example
    ///
    /// ```rust
    /// use framels::paths::Paths;
    /// use std::path::PathBuf;
    ///
    /// let paths = Paths::from(vec![PathBuf::from("foo"), PathBuf::from("bar")]);
    ///
    /// for path in paths.iter() {
    ///    println!("{}", path.to_string_lossy());
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.data.iter()
    }
    pub fn par_iter(&self) -> impl ParallelIterator<Item = &PathBuf> {
        self.data.par_iter()
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

impl Deref for Paths {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.data
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
    pub fn from(data: Vec<PathBuf>) -> Self {
        PathsPacked {
            paths: Paths::from(data),
            metadata: Vec::new(),
        }
    }
    /// Push a [PathBuf] in the the data of the [PathsPacked]
    pub fn push_paths(&mut self, path: PathBuf) {
        self.paths.data.push(path)
    }
    /// Sort the paths attributes
    pub fn sort_paths(&mut self) {
        self.paths.data.sort()
    }
    /// Sort the paths attributes in parallel using [rayon]
    pub fn sort_par_paths(&mut self) {
        self.paths.data.par_sort()
    }
    /// Extend the paths attributes
    pub fn extend_paths(&mut self, paths: Paths) {
        self.paths.data.extend(paths.data)
    }
    /// Return a clone of the paths elements
    pub fn get_paths(&self) -> Paths {
        self.paths.clone()
    }
    /// Push a metadata String
    pub fn push_metadata(&mut self, path: String) {
        self.metadata.push(path)
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

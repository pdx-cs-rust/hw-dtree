//! Directory Tree Simulator: Provides a directory tree structure and an operating system stub
//! structure to interact with it.

// Bart Massey 2021

// Workaround for Clippy false positive in Rust 1.51.0.
// https://github.com/rust-lang/rust-clippy/issues/6546
#![allow(clippy::result_unit_err)]

use thiserror::Error;

/// Errors during directory interaction.
#[derive(Error, Debug)]
pub enum DirError<'a> {
    /// The character `/` in component names is disallowed,
    /// to make path separators easier.
    #[error("{0}: slash in name is invalid")]
    SlashInName(&'a str),
    /// Only one subdirectory of a given name can exist in any directory.
    #[error("{0}: directory exists")]
    DirExists(&'a str),
    /// Traversal failed due to missing subdirectory.
    #[error("{0}: invalid element in path")]
    InvalidChild(&'a str),
}

/// Result type for directory errors.
pub type Result<'a, T> = std::result::Result<T, DirError<'a>>;

/// A directory entry. Component names are stored externally.
#[derive(Debug, Clone)]
pub struct DEnt<'a> {
    pub name: &'a str,
    pub subdir: DTree<'a>,
}

/// A directory tree.
#[derive(Debug, Clone, Default)]
pub struct DTree<'a> {
    pub children: Vec<DEnt<'a>>,
}

/// Operating system state: the directory tree and the current working directory.
#[derive(Debug, Clone, Default)]
pub struct OsState<'a> {
    pub dtree: DTree<'a>,
    pub cwd: Vec<&'a str>,
}

impl<'a> DEnt<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        todo!()
    }
}

impl<'a> DTree<'a> {
    /// Create a new empty directory tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make a subdirectory with the given name in this directory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// assert_eq!(&dt.paths(), &["/test/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<()> {
        todo!()
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// let paths = dt.with_subdir(&["test"], |dt| dt.paths()).unwrap();
    /// assert_eq!(&paths, &["/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir<'b, F, R>(&'b self, path: &[&'a str], f: F) -> Result<R>
    where
        F: FnOnce(&'b DTree<'a>) -> R,
    {
        todo!()
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory
    /// mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// assert_eq!(&dt.paths(), &["/a/b/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir_mut<'b, F, R>(&'b mut self, path: &[&'a str], f: F) -> Result<R>
    where
        F: FnOnce(&'b mut DTree<'a>) -> R,
    {
        todo!()
    }

    /// Produce a list of the paths to each reachable leaf, in no particular order.  Path
    /// components are prefixed by `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("c").unwrap()).unwrap();
    /// let mut paths = dt.paths();
    /// paths.sort();
    /// assert_eq!(&paths, &["/a/b/", "/a/c/"]);
    /// ```
    pub fn paths(&self) -> Vec<String> {
        todo!()
    }
}

impl<'a> OsState<'a> {
    /// Create a new directory tree in the operating system.  Current working directory is the
    /// root.
    pub fn new() -> Self {
        Self::default()
    }

    /// If `path` is empty, change the working directory to the root.  Otherwise change the
    /// working directory to the subdirectory given by `path`.  (There is no notion of `..`:
    /// `path` must be a valid sequence of component names.)
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::OsState;
    /// let mut s = OsState::new();
    /// s.mkdir("a").unwrap();
    /// s.chdir(&["a"]).unwrap();
    /// s.mkdir("b").unwrap();
    /// s.mkdir("c").unwrap();
    /// s.chdir(&[]).unwrap();
    /// assert_eq!(&s.paths().unwrap(), &["/a/b/", "/a/c/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the new working directory is invalid. On error, the original
    /// working directory will be retained.
    pub fn chdir(&mut self, path: &[&'a str]) -> Result<()> {
        todo!()
    }

    /// Make a new subdirectory with the given `name` in the working directory.
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<()> {
        todo!()
    }

    /// Produce a list of the paths from the working directory to each reachable leaf, in no
    /// particular order.  Path components are separated by `/`.
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    pub fn paths(&self) -> Result<Vec<String>> {
        todo!()
    }
}

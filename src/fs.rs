//! This module implements various filesystem wrappers, to guarantee safer fs operations.
//!
//! ```rust
//! todo!("add a simple example showcasing this module functionalities!!!");
//! todo!("add tests for this module");
//! ```

use std::{fs::FileType, path::PathBuf};

use crate::errors::Result;

/// Struct storing an absolute path.
#[derive(Debug)]
pub struct AbsPath {
    path: PathBuf,
}

/// Struct storing a relative path.
#[derive(Debug)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute());
        Self { path: path }
    }

    /// Get canonicalized path.
    pub fn canonicalize(&self) -> Result<Self> {
        Ok(self.path.canonicalize()?.into())
    }

    /// Get relative path.
    pub fn to_relative(&self, prefix: &AbsPath) -> Result<RelPath> {
        Ok(self.path.strip_prefix(&prefix.path)?.to_path_buf().into())
    }

    /// Append to path.
    pub fn join(&self, suffix: &RelPath) -> AbsPath {
        self.path.join(&suffix.path).into()
    }

    /// Get FileType.
    ///
    /// Can also be used to check if path exists, simply by checking if the result is not an error.
    pub fn file_type(&self) -> Result<FileType> {
        Ok(self.path.metadata()?.file_type())
    }
}

impl RelPath {
    /// Creates new RelPath from relative path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_relative());
        Self { path }
    }

    /// Add a prefix to turn relative path into absolute path.
    pub fn to_absolute(&self, base: &AbsPath) -> AbsPath {
        base.path.join(&self.path).into()
    }
}

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.path
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.path
    }
}

impl From<&str> for AbsPath {
    fn from(s: &str) -> Self {
        Self::new(PathBuf::from(s))
    }
}

impl From<&str> for RelPath {
    fn from(s: &str) -> Self {
        Self::new(PathBuf::from(s))
    }
}

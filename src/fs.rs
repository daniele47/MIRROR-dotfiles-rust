//! This module implements various filesystem wrappers, to guarantee safer fs operations.
//!
//! ```rust
//! todo!("add a simple example showcasing this module functionalities!!!");
//! todo!("add tests for this module");
//! ```

use std::{
    fs::{self, File, FileType},
    path::PathBuf,
};

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
    pub fn file_type(&self) -> Result<FileType> {
        Ok(self.path.metadata()?.file_type())
    }

    /// Check if path exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Create directory with all missing parents.
    pub fn create_dir(&self) -> Result<()> {
        Ok(fs::create_dir_all(&self.path)?)
    }

    /// Create file, with all missing parents.
    pub fn create_file(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        File::create(&self.path)?;
        Ok(())
    }

    pub fn delete_dirs(&self) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // keep deleting empty dirs
        let mut curr = self.clone();
        todo!();
        Ok(())
    }

    /// Purge path, whatever file type it is.
    pub fn purge_path(&self) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // delete whatever is in the path
        let path = self.path.canonicalize()?;
        let metadata = path.symlink_metadata()?;
        if metadata.is_dir() {
            fs::remove_dir_all(&self.path)?;
        } else {
            fs::remove_file(&self.path)?;
        }

        // clear empty parent dirs
        self.create_dir()?;
        self.delete_dirs()?;

        Ok(())
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

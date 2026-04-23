//! This module implements various filesystem wrappers, to guarantee safer fs operations.
//!
//! ```rust
//! todo!("add a simple example showcasing this module functionalities!!!");
//! todo!("add tests for this module");
//! ```

use std::{
    env,
    fs::{self, File, FileType},
    path::PathBuf,
};

use crate::errors::Result;

/// Struct storing an absolute path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbsPath {
    path: PathBuf,
}

/// Struct storing a relative path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute());
        Self { path }
    }

    /// Creates a new pseudo-random AbsPath in a temporary location.
    ///
    /// This function should be used mostly for tests!
    ///
    /// Notes:
    /// - this function should be mostly be used for tests, as files in `/tmp` dir in linux
    ///   are often stored directly in ram via tmpfs mount, thus it's not ideal for big files!
    /// - this doesn't guarantee the path doesn't exist, to be safe, this function should
    ///   be used in a loop and a new path should be generated until one doesn't exist.
    ///   But for simple testing porpouses, this function should be good enough, just make sure
    ///   to cleanup the temporary files and directories!
    ///
    /// Implementation details: pseudo-randomicity comes from 3 simple factors:
    /// - prefix passed as a string (more of an identifier, than proper randomness)
    /// - current time in nano seconds (pretty much impossible to repeat twice)
    /// - current process pid (for some extra randomness, which does not hurt)
    pub fn new_tmp(prefix: &str) -> Self {
        let tmp_dir = env::temp_dir();
        let pid = std::process::id();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let tmp_name_str = format!("{}_{}_{}.tmp", prefix, now.as_nanos(), pid);
        let tmp_name = PathBuf::from(&tmp_name_str);
        AbsPath::from(tmp_dir.join(tmp_name))
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
    ///
    /// Notes: there could be some directory left created on failure!
    pub fn create_dir(&self) -> Result<()> {
        if self.exists() && !self.file_type()?.is_dir() {
            self.purge_path(false)?;
        }
        Ok(fs::create_dir_all(&self.path)?)
    }

    /// Create file, with all missing parents.
    ///
    /// Notes:
    /// - There could be some directory left created on failure!
    /// - This is unable to delete not empty dirs, for safety reasons, thus it will fail if path
    ///   has a not empty directory!
    pub fn create_file(&self) -> Result<()> {
        if self.exists() {
            if self.file_type()?.is_file() {
                return Ok(());
            }
            self.purge_path(false)?;
        }
        // note: this parent call is not fully safe, as path could not be normalized beforehand
        // not much i can do differently though ¯\_(ツ)_/¯
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        File::create(&self.path)?;
        Ok(())
    }

    /// Delete empty directory and its anchestors until it finds the first not empty dir!
    pub fn delete_dirs(&self) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // keep deleting empty dirs
        let mut curr = self.canonicalize()?;
        loop {
            if fs::remove_dir(&curr.path).is_err() {
                break;
            }
            let parent = curr.path.parent();
            if parent.is_none() {
                break;
            }
            curr = parent.unwrap().to_path_buf().into();
        }
        Ok(())
    }

    /// Purge path, whatever file type it is.
    ///
    /// DANGER: Use `allowRecursiveDeletion` with caution, as it can easily purge entire
    /// directories!!! Make sure to use with extreme caution always!
    pub fn purge_path(&self, allow_recursive_deletion: bool) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // delete whatever is in the path
        let path = self.path.canonicalize()?;
        if path.symlink_metadata()?.is_dir() {
            if allow_recursive_deletion {
                fs::remove_dir_all(&self.path)?;
            } else {
                fs::remove_dir(&self.path)?;
            }
        } else {
            fs::remove_file(&self.path)?;
        }

        // clear empty parent dirs
        if let Some(parent) = self.path.parent() {
            let abs_parent = AbsPath::new(parent.to_path_buf());
            abs_parent.delete_dirs()?;
        }

        Ok(())
    }

    /// Copy file into destination.
    pub fn copy_file(&self, dst: &AbsPath) -> Result<()> {
        dst.create_file()?;
        fs::copy(&self.path, &dst.path)?;
        Ok(())
    }

    /// List all files in a directory.
    ///
    /// Note: this will get ALL files, even directories, symlinks, all rust can get.
    /// Manual filtering is required when using this function!
    pub fn list_files(&self) -> Result<Vec<AbsPath>> {
        Ok(fs::read_dir(&self.path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| AbsPath::new(entry.path()))
            .collect())
    }

    /// List all files recursively inside a directory.
    ///
    /// Note: this will get ALL files, even directories, symlinks, all rust can get.
    /// Manual filtering is required when using this function!
    ///
    /// Implementation details: this function uses DFS, using an vector as a stack of directories
    /// found but yet to be explored, and an hashset of all paths explored until now.
    /// The hash set allows to easily check if a new directory was already explored, and if so,
    /// avoid exploring it again. This easily resolves all symlink loops that could be created.
    pub fn all_files(&self) -> Result<Vec<AbsPath>> {
        todo!()
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

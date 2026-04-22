use std::path::PathBuf;

#[derive(Debug)]
pub struct AbsPath {
    path: PathBuf,
}

#[derive(Debug)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute());
        Self { path }
    }
}

impl RelPath {
    fn new(path: PathBuf) -> Self {
        assert!(path.is_relative());
        Self { path }
    }
}

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.path
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.path
    }
}

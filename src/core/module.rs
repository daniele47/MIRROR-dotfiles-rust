//! This module implements structs and methods to handle dotfiles modules.

use std::collections::{HashMap, hash_map::Entry};

use crate::core::{errors::Result, fs::{AbsPath, RelPath}};

/// Policy to use for module entries.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ModulePolicy {
    /// Check always both if file doesn't exist and if file differs.
    #[default]
    Track,
    /// Only check if file doesn't exists.
    NotDiff,
    /// Ignore file entirely.
    Ignore,
}

/// Represents a single module entry, aka a path and its policy.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleEntry {
    path: RelPath,
    policy: ModulePolicy,
}

/// Represents a module, aka an orderer list of path with their policies.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module {
    entries: Vec<ModuleEntry>,
}

impl ModulePolicy {
    fn priority(&self) -> u64 {
        // Note: Lower values have higher precedence.
        match self {
            ModulePolicy::Track => 2,
            ModulePolicy::NotDiff => 1,
            ModulePolicy::Ignore => 0,
        }
    }
}

impl ModuleEntry {
    /// Create new entry.
    pub fn new(path: RelPath, policy: ModulePolicy) -> Self {
        Self { path, policy }
    }

    /// Get path.
    pub fn path(&self) -> &RelPath {
        &self.path
    }

    /// Get policy.
    pub fn policy(&self) -> &ModulePolicy {
        &self.policy
    }
}

impl Module {
    /// Create new Module.
    pub fn new(entries: Vec<ModuleEntry>) -> Self {
        Self { entries }
    }

    /// Get all entries.
    pub fn entries(&self) -> &[ModuleEntry] {
        &self.entries
    }

    fn cleanup_paths(paths: Vec<(AbsPath, ModulePolicy)>, base: &AbsPath) -> Result<Self> {
        let mut values = HashMap::<String, (AbsPath, ModulePolicy)>::new();
        let mut entries = vec![];

        // make sure files are unique BASED on canonicalized path
        for path in paths {
            let path_str = String::try_from(path.0.canonicalize()?)?;
            match values.entry(path_str) {
                Entry::Occupied(mut entry) => {
                    let old = entry.get();
                    if path.1.priority() < old.1.priority() {
                        entry.insert((path.0, path.1));
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert((path.0, path.1));
                }
            }
        }

        // collect into proper result type
        for (_, (path, policy)) in values {
            let entry = ModuleEntry::new(path.to_relative(base)?, policy);
            entries.push(entry);
        }

        Ok(Self::new(entries))
    }

    fn resolve_module(&self, base: &AbsPath) -> Result<Vec<(AbsPath, ModulePolicy)>> {
        let mut paths = vec![];
        for raw_entry in &self.entries {
            let raw_abs_path = raw_entry.path.to_absolute(base);
            if raw_abs_path.exists() {
                let metadata = raw_abs_path.metadata()?;
                let mut files = vec![];

                // if path is directory, collect all files within the directory
                if metadata.is_dir() {
                    let all_files = raw_abs_path.all_files()?;
                    for f in all_files {
                        if f.metadata()?.is_file() {
                            files.push((f, raw_entry.policy));
                        }
                    }
                }
                // if path is a file, collect the file itself only
                else if metadata.is_file() {
                    files.push((raw_abs_path, *raw_entry.policy()));
                }
                paths.extend(files);
            }
        }
        Ok(paths)
    }

    /// Resolves raw module into a list of all actual files, relative to `base` as the base directory.
    pub fn resolve(&self, base: &AbsPath) -> Result<Self> {
        Self::cleanup_paths(self.resolve_module(base)?, base)
    }

    /// Sort by path.
    pub fn sort(&mut self) {
        self.entries.sort_by_cached_key(|e| e.path.to_str_lossy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fs::{AbsPath, RelPath};

    fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
        struct Guard(AbsPath);
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = self.0.purge_path(true);
            }
        }
        Guard(tmpdir.clone())
    }

    #[test]
    fn test_resolve() -> Result<()> {
        // Create temp directory
        let tmp = AbsPath::new_tmp("test_resolve");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

        // Create test structure
        let dir1 = tmp.joins(&["dir1"]);
        let dir2 = tmp.joins(&["dir2"]);
        let file1 = tmp.joins(&["file1.txt"]);
        let file2 = dir1.joins(&["file2.txt"]);
        let file3 = dir1.joins(&["file3.txt"]);
        let subdir = dir1.joins(&["subdir"]);
        let file4 = subdir.joins(&["file4.txt"]);

        dir1.create_dir()?;
        dir2.create_dir()?;
        subdir.create_dir()?;
        file1.create_file(false)?;
        file2.create_file(false)?;
        file3.create_file(false)?;
        file4.create_file(false)?;

        // Create module with overlapping entries
        let module = Module::new(vec![
            ModuleEntry::new(RelPath::from("dir1//"), ModulePolicy::Track),
            ModuleEntry::new(RelPath::from("dir1"), ModulePolicy::NotDiff),
            ModuleEntry::new(
                RelPath::from("dir1").joins(&["file3.txt"]),
                ModulePolicy::Track,
            ),
            ModuleEntry::new(
                RelPath::from("dir1").joins(&["subdir"]),
                ModulePolicy::Track,
            ),
            ModuleEntry::new(RelPath::from("file1.txt"), ModulePolicy::Ignore),
        ]);

        // Resolve
        let resolved = module.resolve(&tmp)?;

        // Verify count
        assert_eq!(resolved.entries().len(), 4);

        // Verify a single entry for semplicity
        for entry in resolved.entries() {
            let path_str = entry.path().to_str_lossy();

            match path_str.as_str() {
                "file1.txt" => {
                    assert_eq!(*entry.policy(), ModulePolicy::Ignore);
                }
                _ => {}
            }
        }

        // Cleanup
        tmp.purge_path(true)?;
        Ok(())
    }
}

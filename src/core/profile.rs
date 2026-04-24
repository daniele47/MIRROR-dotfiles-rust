//! This module implements structs and methods to handle dotfiles profiles.

use crate::core::errors::Result;

/// Represents the profile type.
#[derive(Debug, PartialEq, Eq)]
pub enum ProfileType {
    /// Profile storing list of profiles.
    Composite,
    /// Special leaf profile with no children.
    Module,
}

/// Represents a dotfiles profile.
#[derive(Debug)]
pub struct Profile {
    name: String,
    entries: Vec<String>,
    ptype: ProfileType,
}

/// Allow generic implementation of how profiles are loaded.
pub trait ProfileLoader {
    fn load(&mut self, name: &str) -> Result<Profile>;
}

impl Profile {
    /// Create new profile.
    pub fn new(name: String, entries: Vec<String>, ptype: ProfileType) -> Self {
        Self {
            name,
            entries,
            ptype,
        }
    }

    /// Get profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get profile entries.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Get profile type.
    pub fn ptype(&self) -> &ProfileType {
        &self.ptype
    }

    /// Check if profile is resolved, aka all children are modules.
    pub fn is_resolved(&self, loader: &mut impl ProfileLoader) -> bool {
        for child in &self.entries {
            let child_profile = loader.load(child);
            if let Ok(cp) = child_profile {
                if cp.ptype != ProfileType::Module {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Profile Resolver function.
    ///
    /// Profile has 2 implicit states:
    /// - raw: loaded as is from a config files, with possible duplication, with not leaf children
    /// - resolved: cleanup of duplicates, and with all leaf children resolved
    ///
    /// This function serves that role, in trasforming a raw profile into a resolved one.
    pub fn resolve(&mut self, _loader: &mut impl ProfileLoader) -> Self {
        todo!();
    }
}

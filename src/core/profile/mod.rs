//! This module implements structs and methods to handle autosaver profiles.

use crate::core::profile::{composite::Composite, module::Module};

pub mod composite;
pub mod module;

/// Represents the profile type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProfileType {
    /// Profile storing list of profiles.
    Composite(Composite),
    /// Special leaf profile with no children.
    Module(Module),
}

/// Represents a autosaver profile.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Profile {
    name: String,
    ptype: ProfileType,
}

impl Profile {
    /// Create new profile.
    pub fn new(name: String, ptype: ProfileType) -> Self {
        Self { name, ptype }
    }

    /// Get profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get profile type.
    pub fn ptype(&self) -> &ProfileType {
        &self.ptype
    }
}

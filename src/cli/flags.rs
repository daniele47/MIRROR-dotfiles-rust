//! Module to parse cmdline flags.
//!
//! Very simple logic:
//! - parse letter flags, with/without an argument if specified for them
//! - parse word flags, with/without an argument if specified for them
//! - collect remaining arguments into a vector

/// All possible types of a flag.
pub enum FlagType {
    /// Single letter such as `-a`.
    /// Note: `-abc` is the same as `-a -b -c`.
    Letter(char),
    /// Word flags such as `--banana`.
    Word(String),
}

/// Represents a flag.
pub struct Flag {
    /// The type of the flag.
    pub ftype: FlagType,
    /// The index of the flag in the initial cmdline.
    pub index: usize,
    /// The parameter of the flag, if present.
    pub param: Option<String>,
}

/// Represent a not flag parameter.
pub struct Param {
    /// The index of the flag in the initial cmdline.
    pub index: usize,
    /// The actual value of the parameter.
    pub value: String,
}

/// Represent an entire parsed cmdline.
pub struct ParsedArgs {
    /// Stores the cmdline, useful for the various indexes.
    pub cmdline: Vec<String>,
    /// Stores all flags found.
    pub flags: Vec<Flag>,
    /// Stores not flag parameters found.
    pub params: Vec<Param>,
}

//! This module has utilities to parse all kind of profile configuration files.

use crate::core::{module::Module, profile::Profile};

pub enum ParsedConfig {
    Profile(Profile),
    Module(Module),
}

// represent intermidiate parsing step
enum RawKind {
    Option,
    Data,
}

struct RawItem {
    line: u64,
    content: String,
    kind: RawKind,
}

struct RawConfig<I>
where
    I: Iterator<Item = RawItem>,
{
    items: I,
}

// TO BE REMOVED ONCE ALL IS IMPLEMENTED:
//
// Have 3 types of parsers:
// - raw parser (takes iterator of lines and map them to an intermidiate config struct)
// - a parser x each config type (that actually takes the raw parser and correctly creates its proper config type)
// - wrapper parser (handles EVERYTHING. from raw config iterator returns a `ParsedConfig`)
//
// NOTE: THIS ENTIRE PARSING SYSTEM WILL NEVER FULLY ALLOCATE EVERYTHING INTO MEMORY. Just have
// iterator everywhere, until a final ParsedConfig is achieved:
// Iterator<file_lines> -> Iterator<raw_config> -> Fully loaded `ParsedConfig`

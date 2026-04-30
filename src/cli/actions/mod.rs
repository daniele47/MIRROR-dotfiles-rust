//! Module to run cli.

use crate::cli::{
    error::Result,
    flags::{Flag, ParsedArgs},
};

mod version;

/// Struct with data and methods to run cli.
pub struct Runner {
    args: ParsedArgs,
}

impl Runner {
    /// Create new runner.
    pub fn new(args: ParsedArgs) -> Self {
        Self { args }
    }

    /// Run the cli application.
    pub fn run(&mut self) -> Result<()> {
        let flags = self.args.flags();
        let flag_help = flags.contains(&Flag::Word("help".into()));
        let flag_help = flag_help || flags.contains(&Flag::Letter('h'));
        let flag_version = flags.contains(&Flag::Word("version".into()));
        let flag_version = flag_version || flags.contains(&Flag::Letter('v'));
        let flag_nocolor = flags.contains(&Flag::Word("nocolor".into()));

        if flag_version {
            return self.version();
        }

        if let Some(cmd) = self.args.params().first() {
            match cmd.as_str() {
                "list" | "save" | "restore" => todo!(),
                _ => todo!(),
            }
        }

        Ok(())
    }
}

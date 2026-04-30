//! Module to run cli.

use crate::cli::{
    error::{Error, Result},
    flags::{Flag, ParsedArgs},
    output::Renderer,
};

mod backup;
mod help;
mod version;

/// Struct with data and methods to run cli.
pub struct Runner<I>
where
    I: Renderer,
{
    args: ParsedArgs,
    renderer: I,
}

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    /// Create new runner.
    pub fn new(args: ParsedArgs, renderer: I) -> Self {
        Self { args, renderer }
    }

    /// Run the cli application.
    pub fn run(&mut self) -> Result<()> {
        let flags = self.args.flags();
        let wflag_help = flags.contains(&Flag::Word("help".into()));
        let lflag_help = flags.contains(&Flag::Letter('h'));
        let flag_version = flags.contains(&Flag::Word("version".into()));
        let flag_nocolor = flags.contains(&Flag::Word("nocolor".into()));

        if flag_nocolor {
            self.renderer.options().has_colors = false;
        }
        if flag_version {
            return self.version();
        }
        if lflag_help || wflag_help {
            return self.help();
        }

        let command = self.args.params().first().map(|s| s.as_str());
        match command {
            Some("list") | Some("save") | Some("restore") => self.backup(),
            _ => {
                let err_msg = format!("Invalid command '{}'", command.unwrap_or(""));
                Err(Error::EarlyExit(err_msg))
            }
        }
    }
}

use crate::cli::{actions::Runner, error::Result, inout::InOut};

impl<I: InOut> Runner<I> {
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        self.check_flags(&["--help", "--nocolor"])?;
        let first = self.args.params().first().map(String::as_ref).unwrap_or("");
        let _ = self.args.params().get(1).map(String::as_ref).unwrap_or("");
        let _ = self.args.params().get(2).map(String::as_ref).unwrap_or("");
        let col = Self::HELP_COLOR;
        let io = &mut self.inout;
        match first {
            "list" => {}
            "save" => {}
            "restore" => {}
            _ => {
                io.writeln("Commands:", col);
                io.write("  list        ", col);
                io.writeln("Show differences between home and backup files", &[]);
                io.write("  save        ", col);
                io.writeln("Save changes from the home directory to the backup", &[]);
                io.write("  restore     ", col);
                io.writeln("Restore changes from the backup to the home directory", &[]);
                io.write("  delete      ", col);
                io.writeln("Delete files from both home and backup directories", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --help -h   ", col);
                io.writeln("Print the help message for commands and subcommands", &[]);
                io.write("  --version   ", col);
                io.writeln("Print the binary version", &[]);
                io.write("  --nocolor   ", col);
                io.writeln("Avoid all colors in the output", &[]);
            }
        }
        Ok(())
    }
}

use clap::Parser;

use crate::{
    cli::{Cli, CliCmd, PresetSubCmd, config::CliContext},
    errln,
};

impl Cli {
    pub fn action_preset(&self, ctx: &CliContext) -> anyhow::Result<()> {
        if let CliCmd::Preset { preset_subcmd } = self.cmd {
            match preset_subcmd {
                PresetSubCmd::Init {
                    only_scripts,
                    only_dotfiles,
                } => {
                    let no_filter = !only_scripts && !only_dotfiles;
                    if no_filter || only_scripts {
                        let cmd = &["", "run"];
                        Self::run_action(self, cmd, |c| c.action_run(ctx))?;
                    }
                    if no_filter || only_dotfiles {
                        let cmd = &["", "restore", "-cep"];
                        Self::run_action(self, cmd, |c| c.action_backup(ctx))?;
                    }
                }
                PresetSubCmd::Purge => {
                    let cmd = &["", "delete", "-co"];
                    Self::run_action(self, cmd, |c| c.action_backup(ctx))?;
                }
            }
        }
        Ok(())
    }

    fn run_action(
        &self,
        cmd: &[&str],
        run: impl Fn(Cli) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let mut cli = self.clone();
        let parsed = Cli::parse_from(cmd);
        cli.cmd = parsed.cmd;
        errln!("---> Executing '{}' command:", cmd[1..].join(" "));
        run(cli)?;
        errln!();
        Ok(())
    }
}

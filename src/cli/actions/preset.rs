use crate::{
    cli::{
        Cli, CliActBackup, CliActDelSymlinks, CliActSaveRestore, CliCmd, PresetSubCmd,
        config::CliContext,
    },
    errln,
};

impl Cli {
    pub fn action_preset(&self, ctx: &CliContext) -> anyhow::Result<()> {
        if let CliCmd::Preset { preset_subcmd } = self.cmd {
            match preset_subcmd {
                PresetSubCmd::Init => {
                    let mut cli = self.clone();

                    errln!("---> (1/2) Executing 'run' command:");
                    cli.cmd = CliCmd::Run { allow_stdin: true };
                    cli.action_run(ctx)?;

                    errln!("\n---> (2/2) Executing 'restore' command:");
                    cli.cmd = CliCmd::Restore {
                        act_saverestore: CliActSaveRestore {
                            allow_duplicates: false,
                            allow_purge: true,
                        },
                        act_delsymlinks: CliActDelSymlinks {
                            allow_symlink: true,
                        },
                        allow_cleanup: true,
                        act_backup: CliActBackup {
                            show_excluded: true,
                            show_unmodified: false,
                        },
                    };
                    cli.action_backup(ctx)?;
                }
                PresetSubCmd::Purge => {
                    let mut cli = self.clone();

                    errln!("\n---> (1/1) Executing 'delete' command:");
                    cli.cmd = CliCmd::Delete {
                        only_cleanup: true,
                        only_backup: false,
                        only_original: true,
                        act_delsymlinks: CliActDelSymlinks {
                            allow_symlink: true,
                        },
                    };
                    cli.action_backup(ctx)?;
                }
            }
        }
        Ok(())
    }
}

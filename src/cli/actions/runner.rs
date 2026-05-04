use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
        inout::InOut,
    },
    core::profile::{ProfileType, composite::ProfileLoader},
};

impl<I: InOut> Runner<I> {
    /// Backup action to list/save/restore files.
    pub fn runner(&mut self) -> Result<()> {
        // check flags
        self.check_flags(&["--show", "-s", "--list", "-l"])?;

        // get args
        let default_profile = String::new();
        let mut arg_profile = self.args.params().get(2).unwrap_or(&default_profile);
        let env_profile = Self::env("profile").unwrap_or_default();
        let wflag_show = self.args.flags().contains(&Flag::Word("show".into()));
        let lflag_show = self.args.flags().contains(&Flag::Letter('s'));
        let flag_show = wflag_show || lflag_show;
        let wflag_list = self.args.flags().contains(&Flag::Word("list".into()));
        let lflag_list = self.args.flags().contains(&Flag::Letter('l'));
        let flag_list = wflag_list || lflag_list;

        if arg_profile.is_empty() {
            if env_profile.is_empty() {
                return Err(Error::GenericError("No profile specified".into()));
            }
            arg_profile = &env_profile;
        }

        // paths
        let run_dir = Self::paths("run")?;

        // resolve profile into all leafs
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(arg_profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(_) => {}
                ProfileType::Runner(runner) => {}
            }
        }
        Ok(())
    }
}

use crate::core::{error::Result, parsers::RawItem, profile::Profile};

#[derive(Debug)]
pub(super) struct ModuleParser {}

impl ModuleParser {
    pub(super) fn parse(
        profile: String,
        raw: impl Iterator<Item = Result<RawItem>>,
    ) -> Result<Profile> {
        drop(profile);
        drop(raw);
        todo!()
    }
}

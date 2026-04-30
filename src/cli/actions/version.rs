use crate::cli::{
    actions::Runner,
    error::{Error, Result},
    output::Renderer,
};

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    /// Version action to render the binary version.
    pub fn version(&mut self) -> Result<()> {
        let fmt = format!("{} {}", Self::BIN_NAME, Self::CARGO_VERSION);
        self.renderer.writeln(fmt, &[])
    }
}

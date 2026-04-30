use crate::cli::{actions::Runner, error::Result, render::Renderer};

impl<I: Renderer> Runner<I> {
    /// Version action to render the binary version.
    pub fn version(&mut self) -> Result<()> {
        let fmt = format!("{} {}", Self::BIN_NAME, Self::CARGO_VERSION);
        self.renderer.writeln(fmt, &[]);
        Ok(())
    }
}

use crate::cli::{actions::Runner, error::Result, render::Renderer};

impl<I: Renderer> Runner<I> {
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        todo!()
    }
}

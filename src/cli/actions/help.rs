use crate::cli::{actions::Runner, error::Result, inout::InOut};

impl<I: InOut> Runner<I> {
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        todo!()
    }
}

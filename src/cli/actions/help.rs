use crate::cli::{
    actions::Runner,
    error::{Error, Result},
    output::Renderer,
};

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        todo!()
    }
}

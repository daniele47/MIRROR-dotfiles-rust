use crate::cli::{
    actions::Runner,
    error::{Error, Result},
    output::Renderer,
};

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    pub fn help(&mut self) -> Result<()> {
        todo!()
    }
}

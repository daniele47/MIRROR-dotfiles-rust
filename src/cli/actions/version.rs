use crate::cli::{actions::Runner, error::Result};

const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &str = env!("CARGO_PKG_NAME");

impl Runner {
    pub fn version(&mut self) -> Result<()> {
        println!("{BIN_NAME} {CARGO_VERSION}"); // TODO: replace with proper output interface
        Ok(())
    }
}

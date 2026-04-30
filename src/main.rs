use std::env;

use autosaver::cli::{error::Result, flags::ParsedArgs};

fn main() -> Result<()> {
    let args = env::args().collect();
    let parsed_args = ParsedArgs::parse(args);
    println!("{parsed_args:?}");
    Ok(())
}

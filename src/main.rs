use std::{env, process::exit};

use autosaver::cli::{
    actions::Runner,
    flags::ParsedArgs,
    render::{Renderer, RendererOptions, TermRenderer},
};

fn main() {
    // parse cmdline args
    let parsed_args = ParsedArgs::parse(env::args().skip(1).collect());

    // get a frontend renderer
    let mut renderer = TermRenderer::new(RendererOptions::new(true));

    // get cli runner
    let mut runner = Runner::new(parsed_args, renderer.clone());

    // run cli
    if let Err(e) = runner.run() {
        renderer.error(e);
        exit(1);
    }
}

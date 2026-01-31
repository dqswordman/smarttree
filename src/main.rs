use anyhow::Result;
use clap::Parser;

use smarttree::cli::Cli;
use smarttree::config;
use smarttree::discover;
use smarttree::render;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::load(&cli)?;
    let result = discover::discover(&config)?;
    let output = render::render(&result.tree, result.workspace.as_ref(), &config);
    print!("{output}");
    Ok(())
}

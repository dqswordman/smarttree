use clap::Parser;

use smarttree::cli::Cli;
use smarttree::config;
use smarttree::discover;
use smarttree::error::SmarttreeError;
use smarttree::render;

fn main() {
    if let Err(err) = run() {
        eprintln!("smarttree: {err}");
        if let Some(hint) = hint_for_error(&err) {
            eprintln!("hint: {hint}");
        }
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.init {
        let outcome = config::init_config(&cli)?;
        if outcome.created {
            println!("Created config at {}", outcome.path.display());
            println!("Edit it to customize, or run `smarttree` to see output.");
        } else {
            println!("Config already exists at {}", outcome.path.display());
            println!("Edit it, or pass --config to create a new one elsewhere.");
        }
        return Ok(());
    }

    let config = config::load(&cli)?;
    let result = discover::discover(&config)?;
    let output = render::render(&result.tree, result.workspace.as_ref(), &config);
    print!("{output}");
    Ok(())
}

fn hint_for_error(err: &anyhow::Error) -> Option<String> {
    let smart = err.downcast_ref::<SmarttreeError>()?;
    match smart {
        SmarttreeError::ConfigRead { path, .. } => Some(format!(
            "Config file not readable at {}. Run `smarttree --init` to create one.",
            path.display()
        )),
        SmarttreeError::ConfigParse { path, .. } => Some(format!(
            "Config file at {} is not valid YAML. Fix the file or re-run `smarttree --init`.",
            path.display()
        )),
        SmarttreeError::ConfigWrite { path, .. } => Some(format!(
            "Unable to write config at {}. Check permissions or choose a different path with --config.",
            path.display()
        )),
        _ => None,
    }
}

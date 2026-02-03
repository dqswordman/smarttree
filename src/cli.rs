use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::model::{Format, Lens};

#[derive(Parser, Debug)]
#[command(
    name = "smarttree",
    version,
    about = "Project-aware tree output",
    after_help = "Examples:\n  smarttree\n  smarttree . --lens files --depth 3\n  smarttree --format md --max-children 60\n  smarttree --init\n"
)]
pub struct Cli {
    /// Path to scan (default: current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Output lens: module or files
    #[arg(long, value_enum)]
    pub lens: Option<Lens>,

    /// Output format: text or md
    #[arg(long, value_enum)]
    pub format: Option<Format>,

    /// Max depth to traverse
    #[arg(long)]
    pub depth: Option<usize>,

    /// Max total items to visit
    #[arg(long = "max-items")]
    pub max_items: Option<usize>,

    /// Max children per directory to display
    #[arg(long = "max-children")]
    pub max_children: Option<usize>,

    /// Respect .gitignore and related files
    #[arg(long = "respect-gitignore", action = ArgAction::SetTrue)]
    pub respect_gitignore: bool,

    /// Do not respect .gitignore and related files
    #[arg(long = "no-respect-gitignore", action = ArgAction::SetTrue)]
    pub no_respect_gitignore: bool,

    /// Path to config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Disable config file loading
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_config: bool,

    /// Create a default .smarttree.yaml in the target directory and exit
    #[arg(long, action = ArgAction::SetTrue)]
    pub init: bool,

    /// Add an ignore pattern (repeatable)
    #[arg(long = "ignore", value_name = "PATTERN")]
    pub ignore: Vec<String>,

    /// Add an include pattern that overrides ignore (repeatable)
    #[arg(long = "include", value_name = "PATTERN")]
    pub include: Vec<String>,

    /// Show hidden files and directories
    #[arg(long, action = ArgAction::SetTrue)]
    pub hidden: bool,

    /// Use Unicode tree characters
    #[arg(long, action = ArgAction::SetTrue)]
    pub unicode: bool,

    /// Use ASCII tree characters
    #[arg(long, action = ArgAction::SetTrue)]
    pub ascii: bool,
}

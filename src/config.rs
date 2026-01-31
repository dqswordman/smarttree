use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;

use crate::cli::Cli;
use crate::error::SmarttreeError;
use crate::model::{Format, Lens};

const DEFAULT_DEPTH: usize = 4;
const DEFAULT_MAX_ITEMS: usize = 20_000;
const DEFAULT_MAX_CHILDREN: usize = 200;
const DEFAULT_RESPECT_GITIGNORE: bool = true;
const DEFAULT_HIDDEN: bool = false;
const DEFAULT_UNICODE: bool = false;

const DEFAULT_KEY_DIRS: &[&str] = &[
    "src", "tests", "test", "docs", "examples", "scripts", "public", "include", "cmd", "bin",
];

const DEFAULT_IGNORE: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    "out",
    ".next",
    ".turbo",
    ".cache",
    ".pnpm-store",
    "coverage",
    ".venv",
    "venv",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "target",
    ".idea",
    ".vscode",
    ".DS_Store",
    "Thumbs.db",
];

#[derive(Debug, Clone)]
pub struct Config {
    pub root: PathBuf,
    pub lens: Lens,
    pub format: Format,
    pub depth: usize,
    pub max_items: usize,
    pub max_children: usize,
    pub respect_gitignore: bool,
    pub hidden: bool,
    pub unicode: bool,
    pub ignore: Vec<String>,
    pub include: Vec<String>,
    pub key_dirs: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    pub lens: Option<Lens>,
    pub format: Option<Format>,
    pub depth: Option<usize>,
    pub max_items: Option<usize>,
    pub max_children: Option<usize>,
    pub respect_gitignore: Option<bool>,
    pub hidden: Option<bool>,
    pub unicode: Option<bool>,
    pub ignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub key_dirs: Option<Vec<String>>,
}

fn resolve_root(path: &Path) -> PathBuf {
    match fs::metadata(path) {
        Ok(meta) if meta.is_file() => path.parent().unwrap_or(path).to_path_buf(),
        _ => path.to_path_buf(),
    }
}

fn find_config_file(root: &Path) -> Option<PathBuf> {
    let candidates = [
        root.join(".smarttree.yaml"),
        root.join(".smarttree.yml"),
        root.join("smarttree.yaml"),
    ];
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn load_config_file(path: &Path) -> Result<ConfigFile> {
    let content = fs::read_to_string(path).map_err(|source| SmarttreeError::ConfigRead {
        path: path.to_path_buf(),
        source,
    })?;
    let config = serde_yaml::from_str(&content).map_err(|source| SmarttreeError::ConfigParse {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(config)
}

pub fn load(cli: &Cli) -> Result<Config> {
    let root = cli.path.clone();
    let config_root = resolve_root(&cli.path);

    let config_file = if cli.no_config {
        None
    } else if let Some(path) = &cli.config {
        Some(path.clone())
    } else {
        find_config_file(&config_root)
    };

    let file_config = match config_file {
        Some(path) => load_config_file(&path)?,
        None => ConfigFile::default(),
    };

    let lens = cli.lens.or(file_config.lens).unwrap_or(Lens::Module);
    let format = cli.format.or(file_config.format).unwrap_or(Format::Text);
    let depth = cli.depth.or(file_config.depth).unwrap_or(DEFAULT_DEPTH);
    let max_items = cli
        .max_items
        .or(file_config.max_items)
        .unwrap_or(DEFAULT_MAX_ITEMS);
    let max_children = cli
        .max_children
        .or(file_config.max_children)
        .unwrap_or(DEFAULT_MAX_CHILDREN);

    let respect_gitignore = if cli.no_respect_gitignore {
        false
    } else if cli.respect_gitignore {
        true
    } else {
        file_config
            .respect_gitignore
            .unwrap_or(DEFAULT_RESPECT_GITIGNORE)
    };

    let hidden = if cli.hidden {
        true
    } else {
        file_config.hidden.unwrap_or(DEFAULT_HIDDEN)
    };

    let unicode = if cli.ascii {
        false
    } else if cli.unicode {
        true
    } else {
        file_config.unicode.unwrap_or(DEFAULT_UNICODE)
    };

    let mut ignore = Vec::new();
    ignore.extend(DEFAULT_IGNORE.iter().map(|s| s.to_string()));
    if let Some(extra) = file_config.ignore {
        ignore.extend(extra);
    }
    ignore.extend(cli.ignore.iter().cloned());

    let mut include = Vec::new();
    if let Some(extra) = file_config.include {
        include.extend(extra);
    }
    include.extend(cli.include.iter().cloned());

    let key_dirs = if let Some(list) = file_config.key_dirs {
        list
    } else {
        DEFAULT_KEY_DIRS.iter().map(|s| s.to_string()).collect()
    };

    Ok(Config {
        root,
        lens,
        format,
        depth,
        max_items,
        max_children,
        respect_gitignore,
        hidden,
        unicode,
        ignore,
        include,
        key_dirs,
    })
}

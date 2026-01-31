use std::path::PathBuf;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Lens {
    Module,
    Files,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Text,
    Md,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    Dir,
    File,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ModuleKind {
    Node,
    Python,
    Rust,
    Go,
    Java,
    DotNet,
    Unknown,
}

impl ModuleKind {
    pub fn tag(self) -> &'static str {
        match self {
            ModuleKind::Node => "[node]",
            ModuleKind::Python => "[py]",
            ModuleKind::Rust => "[rs]",
            ModuleKind::Go => "[go]",
            ModuleKind::Java => "[java]",
            ModuleKind::DotNet => "[dotnet]",
            ModuleKind::Unknown => "[module]",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub kind: ModuleKind,
    pub summary: Option<String>,
    pub markers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub rel_path: PathBuf,
    pub kind: NodeKind,
    pub children: Vec<usize>,
    pub module: Option<ModuleInfo>,
    pub error: Option<String>,
}

impl Node {
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, NodeKind::Dir)
    }
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub root_path: PathBuf,
    pub root: usize,
    pub nodes: Vec<Node>,
    pub truncated: bool,
    pub truncated_at: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceKind {
    Pnpm,
    Npm,
    Lerna,
    Cargo,
    Go,
    Turbo,
    Nx,
}

impl WorkspaceKind {
    pub fn label(self) -> &'static str {
        match self {
            WorkspaceKind::Pnpm => "pnpm",
            WorkspaceKind::Npm => "npm",
            WorkspaceKind::Lerna => "lerna",
            WorkspaceKind::Cargo => "cargo",
            WorkspaceKind::Go => "go",
            WorkspaceKind::Turbo => "turbo",
            WorkspaceKind::Nx => "nx",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorkspaceInfo {
    pub kind: WorkspaceKind,
    pub patterns: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct WorkspaceResolved {
    pub kind: WorkspaceKind,
    pub package_roots: Vec<PathBuf>,
}

use anyhow::Result;

use crate::config::Config;
use crate::discover::summary;
use crate::model::{ModuleInfo, ModuleKind, NodeKind, Tree, WorkspaceResolved};

#[derive(Debug, Clone)]
pub struct ModuleCandidate {
    pub node_id: usize,
    pub kind: ModuleKind,
    pub markers: Vec<String>,
}

fn marker_kind_for_file(name: &str) -> Option<ModuleKind> {
    match name {
        "package.json" => Some(ModuleKind::Node),
        "pyproject.toml" | "setup.py" | "setup.cfg" => Some(ModuleKind::Python),
        "Cargo.toml" => Some(ModuleKind::Rust),
        "go.mod" => Some(ModuleKind::Go),
        "pom.xml" | "build.gradle" | "build.gradle.kts" => Some(ModuleKind::Java),
        _ => {
            if name.ends_with(".csproj") {
                Some(ModuleKind::DotNet)
            } else {
                None
            }
        }
    }
}

fn kind_priority(kind: &ModuleKind) -> u8 {
    match kind {
        ModuleKind::Node => 0,
        ModuleKind::Python => 1,
        ModuleKind::Rust => 2,
        ModuleKind::Go => 3,
        ModuleKind::Java => 4,
        ModuleKind::DotNet => 5,
        ModuleKind::Unknown => 6,
    }
}

pub fn collect_module_candidates(tree: &Tree) -> Vec<ModuleCandidate> {
    let mut candidates = Vec::new();
    for (node_id, node) in tree.nodes.iter().enumerate() {
        if node.kind != NodeKind::Dir {
            continue;
        }
        let mut markers = Vec::new();
        let mut kinds = Vec::new();
        for child_id in &node.children {
            let child = &tree.nodes[*child_id];
            if child.kind == NodeKind::Dir {
                continue;
            }
            if let Some(kind) = marker_kind_for_file(&child.name) {
                markers.push(child.name.clone());
                kinds.push(kind);
            }
        }

        if !markers.is_empty() {
            let kind = kinds
                .into_iter()
                .min_by_key(kind_priority)
                .unwrap_or(ModuleKind::Unknown);
            candidates.push(ModuleCandidate {
                node_id,
                kind,
                markers,
            });
        }
    }
    candidates
}

fn is_within_any(path: &std::path::Path, roots: &[std::path::PathBuf]) -> bool {
    roots.iter().any(|root| path.starts_with(root))
}

pub fn annotate_modules(
    tree: &mut Tree,
    candidates: &[ModuleCandidate],
    workspace: Option<&WorkspaceResolved>,
    _config: &Config,
) -> Result<()> {
    let package_roots = workspace.map(|w| w.package_roots.as_slice()).unwrap_or(&[]);
    let restrict_to_workspace = !package_roots.is_empty();

    for candidate in candidates {
        let rel = tree.nodes[candidate.node_id].rel_path.clone();
        let is_root = rel.as_os_str().is_empty();
        if restrict_to_workspace && !is_root && !is_within_any(&rel, package_roots) {
            continue;
        }

        let module_path = tree.root_path.join(&rel);
        let summary = summary::read_summary(&module_path, candidate.kind);
        tree.nodes[candidate.node_id].module = Some(ModuleInfo {
            kind: candidate.kind,
            summary,
            markers: candidate.markers.clone(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Node, NodeKind, Tree};

    #[test]
    fn detects_node_module_marker() {
        let root = Node {
            name: "root".to_string(),
            rel_path: std::path::PathBuf::new(),
            kind: NodeKind::Dir,
            children: vec![1],
            module: None,
            error: None,
        };
        let pkg = Node {
            name: "package.json".to_string(),
            rel_path: std::path::PathBuf::from("package.json"),
            kind: NodeKind::File,
            children: vec![],
            module: None,
            error: None,
        };
        let tree = Tree {
            root_path: std::path::PathBuf::new(),
            root: 0,
            nodes: vec![root, pkg],
            truncated: false,
            truncated_at: 0,
        };

        let candidates = collect_module_candidates(&tree);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].kind, ModuleKind::Node);
        assert_eq!(candidates[0].markers, vec!["package.json".to_string()]);
    }
}

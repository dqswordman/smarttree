use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Deserialize;

use crate::discover::markers::ModuleCandidate;
use crate::error::SmarttreeError;
use crate::model::Tree;
use crate::model::{WorkspaceInfo, WorkspaceKind};

#[derive(Debug, Deserialize)]
struct PnpmWorkspace {
    packages: Option<Vec<String>>,
}

pub fn detect_workspace(root: &Path) -> Result<Option<WorkspaceInfo>> {
    let pnpm = root.join("pnpm-workspace.yaml");
    if pnpm.is_file() {
        let content = fs::read_to_string(&pnpm).unwrap_or_default();
        let doc: PnpmWorkspace =
            serde_yaml::from_str(&content).unwrap_or(PnpmWorkspace { packages: None });
        return Ok(Some(WorkspaceInfo {
            kind: WorkspaceKind::Pnpm,
            patterns: doc.packages.unwrap_or_default(),
        }));
    }

    let package_json = root.join("package.json");
    if package_json.is_file() {
        if let Some(patterns) = parse_package_json_workspaces(&package_json) {
            return Ok(Some(WorkspaceInfo {
                kind: WorkspaceKind::Npm,
                patterns,
            }));
        }
    }

    let lerna = root.join("lerna.json");
    if lerna.is_file() {
        if let Some(patterns) = parse_lerna_packages(&lerna) {
            return Ok(Some(WorkspaceInfo {
                kind: WorkspaceKind::Lerna,
                patterns,
            }));
        }
    }

    let cargo = root.join("Cargo.toml");
    if cargo.is_file() {
        if let Some(patterns) = parse_cargo_workspace(&cargo) {
            return Ok(Some(WorkspaceInfo {
                kind: WorkspaceKind::Cargo,
                patterns,
            }));
        }
    }

    let go_work = root.join("go.work");
    if go_work.is_file() {
        let patterns = parse_go_work(&go_work);
        return Ok(Some(WorkspaceInfo {
            kind: WorkspaceKind::Go,
            patterns,
        }));
    }

    if root.join("turbo.json").is_file() {
        return Ok(Some(WorkspaceInfo {
            kind: WorkspaceKind::Turbo,
            patterns: Vec::new(),
        }));
    }

    if root.join("nx.json").is_file() {
        return Ok(Some(WorkspaceInfo {
            kind: WorkspaceKind::Nx,
            patterns: Vec::new(),
        }));
    }

    Ok(None)
}

fn parse_package_json_workspaces(path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let workspaces = value.get("workspaces")?;
    if let Some(arr) = workspaces.as_array() {
        let patterns = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<_>>();
        return Some(patterns);
    }
    if let Some(obj) = workspaces.as_object() {
        if let Some(arr) = obj.get("packages").and_then(|v| v.as_array()) {
            let patterns = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>();
            return Some(patterns);
        }
    }
    None
}

fn parse_lerna_packages(path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let packages = value.get("packages")?.as_array()?;
    Some(
        packages
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
    )
}

fn parse_cargo_workspace(path: &Path) -> Option<Vec<String>> {
    let content = fs::read_to_string(path).ok()?;
    let value: toml::Value = toml::from_str(&content).ok()?;
    let workspace = value.get("workspace")?;
    let members = workspace
        .get("members")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let patterns = members
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();
    Some(patterns)
}

fn parse_go_work(path: &Path) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut patterns = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("use ") {
            let rest = trimmed.trim_start_matches("use").trim();
            if rest.starts_with('(') {
                in_block = true;
                continue;
            }
            if !rest.is_empty() {
                patterns.push(rest.to_string());
            }
            continue;
        }
        if in_block {
            if trimmed.starts_with(')') {
                in_block = false;
                continue;
            }
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            patterns.push(trimmed.to_string());
        }
    }

    patterns
}

fn normalize_pattern(pattern: &str) -> String {
    let trimmed = pattern.trim();
    trimmed
        .trim_start_matches("./")
        .trim_start_matches('/')
        .to_string()
}

fn build_globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let normalized = normalize_pattern(pattern);
        let glob = Glob::new(&normalized).map_err(|source| SmarttreeError::InvalidPattern {
            pattern: normalized.clone(),
            source,
        })?;
        builder.add(glob);
    }
    Ok(builder.build()?)
}

fn path_to_slash(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn roots_from_patterns(tree: &Tree, patterns: &[String]) -> Result<Vec<PathBuf>> {
    let globset = build_globset(patterns)?;
    let mut roots: HashSet<PathBuf> = HashSet::new();

    for node in &tree.nodes {
        let rel = path_to_slash(&node.rel_path);
        if rel.is_empty() {
            continue;
        }
        if globset.is_match(&rel) {
            if node.kind == crate::model::NodeKind::Dir {
                roots.insert(node.rel_path.clone());
            } else if let Some(parent) = node.rel_path.parent() {
                roots.insert(parent.to_path_buf());
            }
        }
    }

    Ok(roots.into_iter().collect())
}

fn heuristic_package_roots(tree: &Tree, candidates: &[ModuleCandidate]) -> Vec<PathBuf> {
    const GROUP_DIRS: [&str; 4] = ["packages", "apps", "services", "libs"];
    let mut roots = HashSet::new();
    for candidate in candidates {
        let rel = &tree.nodes[candidate.node_id].rel_path;
        let mut comps = rel.components();
        let Some(first) = comps.next() else { continue };
        let Some(first_str) = first.as_os_str().to_str() else {
            continue;
        };
        if !GROUP_DIRS.contains(&first_str) {
            continue;
        }
        if comps.next().is_none() {
            continue;
        }
        roots.insert(rel.clone());
    }
    roots.into_iter().collect()
}

pub fn resolve_package_roots(
    tree: &Tree,
    info: &WorkspaceInfo,
    candidates: &[ModuleCandidate],
) -> Result<Vec<PathBuf>> {
    let mut roots = if info.patterns.is_empty() {
        Vec::new()
    } else {
        roots_from_patterns(tree, &info.patterns)?
    };

    if roots.is_empty() {
        roots = heuristic_package_roots(tree, candidates);
    }

    Ok(roots)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_package_json_workspaces_array() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("package.json");
        fs::write(&path, r#"{ "workspaces": ["apps/*", "packages/*"] }"#).expect("write");
        let patterns = parse_package_json_workspaces(&path).expect("patterns");
        assert_eq!(patterns, vec!["apps/*", "packages/*"]);
    }

    #[test]
    fn parse_package_json_workspaces_object() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("package.json");
        fs::write(&path, r#"{ "workspaces": { "packages": ["libs/*"] } }"#).expect("write");
        let patterns = parse_package_json_workspaces(&path).expect("patterns");
        assert_eq!(patterns, vec!["libs/*"]);
    }

    #[test]
    fn parse_go_work_use_block() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("go.work");
        fs::write(
            &path,
            r#"go 1.20
use (
    ./module-a
    ./module-b
)
"#,
        )
        .expect("write");
        let patterns = parse_go_work(&path);
        assert_eq!(patterns, vec!["./module-a", "./module-b"]);
    }
}

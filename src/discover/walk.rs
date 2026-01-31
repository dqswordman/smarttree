use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::overrides::{Override, OverrideBuilder};
use ignore::WalkBuilder;

use crate::config::Config;
use crate::model::{Node, NodeKind, Tree};
use anyhow::Context;

fn build_overrides(config: &Config) -> Result<Override> {
    let mut builder = OverrideBuilder::new(&config.root);

    for pattern in &config.ignore {
        let ignore = format!("!{pattern}");
        builder
            .add(&ignore)
            .with_context(|| format!("invalid ignore pattern '{pattern}'"))?;
    }

    for pattern in &config.include {
        builder
            .add(pattern)
            .with_context(|| format!("invalid include pattern '{pattern}'"))?;
    }

    Ok(builder.build()?)
}

fn display_root_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

fn single_file_tree(path: &Path) -> Tree {
    let name = display_root_name(path);
    let node = Node {
        name,
        rel_path: PathBuf::new(),
        kind: NodeKind::File,
        children: Vec::new(),
        module: None,
        error: None,
    };
    Tree {
        root_path: path.to_path_buf(),
        root: 0,
        nodes: vec![node],
        truncated: false,
        truncated_at: 1,
    }
}

pub fn build_tree(config: &Config) -> Result<Tree> {
    let root_path = config.root.clone();

    if let Ok(meta) = fs::metadata(&root_path) {
        if meta.is_file() {
            return Ok(single_file_tree(&root_path));
        }
    }

    let overrides = build_overrides(config)?;
    let mut builder = WalkBuilder::new(&root_path);
    builder
        .follow_links(false)
        .hidden(!config.hidden)
        .max_depth(Some(config.depth))
        .overrides(overrides)
        .ignore(config.respect_gitignore)
        .git_ignore(config.respect_gitignore)
        .git_exclude(config.respect_gitignore)
        .git_global(config.respect_gitignore)
        .parents(config.respect_gitignore);

    let mut nodes: Vec<Node> = Vec::new();
    let mut index: HashMap<PathBuf, usize> = HashMap::new();

    let root_node = Node {
        name: display_root_name(&root_path),
        rel_path: PathBuf::new(),
        kind: NodeKind::Dir,
        children: Vec::new(),
        module: None,
        error: None,
    };
    nodes.push(root_node);
    index.insert(PathBuf::new(), 0);

    let mut count = 0usize;
    let mut truncated = false;

    for entry_result in builder.build() {
        if truncated {
            break;
        }
        match entry_result {
            Ok(entry) => {
                let path = entry.path();
                if path == root_path {
                    continue;
                }
                if count >= config.max_items {
                    truncated = true;
                    break;
                }

                let rel_path = match path.strip_prefix(&root_path) {
                    Ok(rel) => rel.to_path_buf(),
                    Err(_) => path.to_path_buf(),
                };
                let name = entry.file_name().to_string_lossy().into_owned();
                let kind = match entry.file_type() {
                    Some(ft) if ft.is_dir() => NodeKind::Dir,
                    Some(_) => NodeKind::File,
                    None => NodeKind::File,
                };

                let node_id = nodes.len();
                nodes.push(Node {
                    name,
                    rel_path: rel_path.clone(),
                    kind,
                    children: Vec::new(),
                    module: None,
                    error: None,
                });
                index.insert(rel_path.clone(), node_id);

                let parent_rel = rel_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(PathBuf::new);
                let parent_id = index.get(&parent_rel).copied().unwrap_or(0);
                nodes[parent_id].children.push(node_id);

                count += 1;
            }
            Err(err) => {
                let Some(path) = error_path(&err) else {
                    continue;
                };
                if path == root_path {
                    continue;
                }
                if count >= config.max_items {
                    truncated = true;
                    break;
                }

                let rel_path = match path.strip_prefix(&root_path) {
                    Ok(rel) => rel.to_path_buf(),
                    Err(_) => path.to_path_buf(),
                };
                let name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| rel_path.to_string_lossy().into_owned());
                let message = err
                    .io_error()
                    .map(|io| match io.kind() {
                        std::io::ErrorKind::PermissionDenied => "permission denied".to_string(),
                        _ => io.to_string(),
                    })
                    .unwrap_or_else(|| err.to_string());

                let node_id = nodes.len();
                nodes.push(Node {
                    name,
                    rel_path: rel_path.clone(),
                    kind: NodeKind::Error,
                    children: Vec::new(),
                    module: None,
                    error: Some(message),
                });
                index.insert(rel_path.clone(), node_id);

                let parent_rel = rel_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(PathBuf::new);
                let parent_id = index.get(&parent_rel).copied().unwrap_or(0);
                nodes[parent_id].children.push(node_id);

                count += 1;
            }
        }
    }

    Ok(Tree {
        root_path,
        root: 0,
        nodes,
        truncated,
        truncated_at: count,
    })
}

fn error_path(err: &ignore::Error) -> Option<&Path> {
    match err {
        ignore::Error::WithPath { path, .. } => Some(path.as_path()),
        ignore::Error::WithLineNumber { err, .. } => error_path(err),
        ignore::Error::WithDepth { err, .. } => error_path(err),
        ignore::Error::Partial(errs) => errs.iter().find_map(error_path),
        ignore::Error::Loop { child, .. } => Some(child.as_path()),
        _ => None,
    }
}

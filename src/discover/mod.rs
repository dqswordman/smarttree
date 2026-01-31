pub mod markers;
pub mod summary;
pub mod walk;
pub mod workspace;

use anyhow::Result;

use crate::config::Config;
use crate::model::{Tree, WorkspaceResolved};

#[derive(Debug)]
pub struct DiscoverResult {
    pub tree: Tree,
    pub workspace: Option<WorkspaceResolved>,
}

pub fn discover(config: &Config) -> Result<DiscoverResult> {
    let mut tree = walk::build_tree(config)?;

    let workspace_root = if std::fs::metadata(&config.root)
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        config.root.parent().unwrap_or(&config.root).to_path_buf()
    } else {
        config.root.clone()
    };

    let workspace_info = workspace::detect_workspace(&workspace_root)?;
    let module_candidates = markers::collect_module_candidates(&tree);

    let workspace_resolved = if let Some(info) = workspace_info {
        let package_roots = workspace::resolve_package_roots(&tree, &info, &module_candidates)?;
        Some(WorkspaceResolved {
            kind: info.kind,
            package_roots,
        })
    } else {
        None
    };

    markers::annotate_modules(
        &mut tree,
        &module_candidates,
        workspace_resolved.as_ref(),
        config,
    )?;

    Ok(DiscoverResult {
        tree,
        workspace: workspace_resolved,
    })
}

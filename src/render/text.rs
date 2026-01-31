use std::collections::HashSet;
use std::path::Path;

use crate::config::Config;
use crate::model::{Lens, NodeKind, Tree, WorkspaceResolved};

#[derive(Clone, Copy)]
struct TreeChars {
    mid: &'static str,
    end: &'static str,
    vert: &'static str,
    space: &'static str,
}

fn chars(unicode: bool) -> TreeChars {
    if unicode {
        TreeChars {
            mid: "\u{251C}\u{2500} ",
            end: "\u{2514}\u{2500} ",
            vert: "\u{2502}  ",
            space: "   ",
        }
    } else {
        TreeChars {
            mid: "|-- ",
            end: "`-- ",
            vert: "|   ",
            space: "    ",
        }
    }
}

fn path_to_slash(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn compute_module_paths(tree: &Tree) -> HashSet<String> {
    let mut paths = HashSet::new();
    paths.insert(String::new());
    for node in &tree.nodes {
        if node.module.is_none() {
            continue;
        }
        let mut current = node.rel_path.clone();
        loop {
            paths.insert(path_to_slash(&current));
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
    }
    paths
}

struct RenderContext<'a> {
    tree: &'a Tree,
    chars: TreeChars,
    module_paths: HashSet<String>,
    key_dirs: HashSet<String>,
    config: &'a Config,
}

pub fn render_text(tree: &Tree, workspace: Option<&WorkspaceResolved>, config: &Config) -> String {
    let module_paths = if config.lens == Lens::Module {
        compute_module_paths(tree)
    } else {
        HashSet::new()
    };
    let key_dirs: HashSet<String> = config.key_dirs.iter().cloned().collect();
    let ctx = RenderContext {
        tree,
        chars: chars(config.unicode),
        module_paths,
        key_dirs,
        config,
    };

    let mut lines = Vec::new();
    let root = &tree.nodes[tree.root];
    lines.push(format_root_label(root, workspace));
    ctx.render_children(tree.root, "", &mut lines);

    if tree.truncated {
        lines.push(format!(
            "... (truncated after reaching max-items={})",
            config.max_items
        ));
    }

    lines.join("\n")
}

impl<'a> RenderContext<'a> {
    fn render_children(&self, node_id: usize, prefix: &str, lines: &mut Vec<String>) {
        let children = self.select_children(node_id);
        if children.is_empty() {
            return;
        }

        let omitted = if children.len() > self.config.max_children {
            children.len() - self.config.max_children
        } else {
            0
        };
        let display_count = if omitted > 0 {
            self.config.max_children
        } else {
            children.len()
        };

        for (idx, child_id) in children.iter().take(display_count).enumerate() {
            let is_last = omitted == 0 && idx == display_count - 1;
            let child = &self.tree.nodes[*child_id];
            let connector = if is_last {
                self.chars.end
            } else {
                self.chars.mid
            };
            let label = format_node_label(child);
            lines.push(format!("{prefix}{connector}{label}"));
            let new_prefix = if is_last {
                format!("{prefix}{}", self.chars.space)
            } else {
                format!("{prefix}{}", self.chars.vert)
            };
            if child.kind == NodeKind::Dir {
                self.render_children(*child_id, &new_prefix, lines);
            }
        }

        if omitted > 0 {
            let connector = self.chars.end;
            let more_label = format!("... ({omitted} more)");
            lines.push(format!("{prefix}{connector}{more_label}"));
        }
    }

    fn select_children(&self, node_id: usize) -> Vec<usize> {
        let parent = &self.tree.nodes[node_id];
        let parent_is_module = parent.module.is_some();
        let marker_files: HashSet<String> = parent
            .module
            .as_ref()
            .map(|m| m.markers.clone())
            .unwrap_or_default()
            .into_iter()
            .collect();

        let mut children: Vec<usize> = parent.children.clone();
        children.retain(|child_id| {
            if self.config.lens == Lens::Files {
                return true;
            }
            let child = &self.tree.nodes[*child_id];
            let child_path = path_to_slash(&child.rel_path);
            match child.kind {
                NodeKind::Dir => {
                    let is_key_dir = parent_is_module && self.key_dirs.contains(&child.name);
                    let in_module_path = self.module_paths.contains(&child_path);
                    is_key_dir || in_module_path
                }
                NodeKind::File | NodeKind::Error => {
                    if parent_is_module {
                        marker_files.contains(&child.name)
                    } else {
                        false
                    }
                }
            }
        });

        children.sort_by(|a, b| {
            let na = &self.tree.nodes[*a];
            let nb = &self.tree.nodes[*b];
            let ka = kind_rank(na.kind);
            let kb = kind_rank(nb.kind);
            ka.cmp(&kb)
                .then_with(|| na.name.to_lowercase().cmp(&nb.name.to_lowercase()))
                .then_with(|| na.name.cmp(&nb.name))
        });

        children
    }
}

fn format_root_label(node: &crate::model::Node, workspace: Option<&WorkspaceResolved>) -> String {
    let mut label = if node.kind == NodeKind::Dir {
        format!("{}/", node.name)
    } else {
        node.name.clone()
    };

    if let Some(workspace) = workspace {
        label.push_str(&format!("  [workspace: {}]", workspace.kind.label()));
    }

    if let Some(module) = &node.module {
        label.push_str(&format!("  {}", module.kind.tag()));
        if let Some(summary) = &module.summary {
            label.push_str(&format!("  {}", summary));
        }
    }

    label
}

fn format_node_label(node: &crate::model::Node) -> String {
    let mut base = if node.kind == NodeKind::Dir {
        format!("{}/", node.name)
    } else if node.kind == NodeKind::Error {
        if let Some(err) = &node.error {
            format!("{} ({})", node.name, err)
        } else {
            node.name.clone()
        }
    } else {
        node.name.clone()
    };

    if let Some(module) = &node.module {
        base.push_str(&format!("  {}", module.kind.tag()));
        if let Some(summary) = &module.summary {
            base.push_str(&format!("  {}", summary));
        }
    }

    base
}

fn kind_rank(kind: NodeKind) -> u8 {
    match kind {
        NodeKind::Dir => 0,
        NodeKind::File | NodeKind::Error => 1,
    }
}

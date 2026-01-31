pub mod md;
pub mod text;

use crate::config::Config;
use crate::model::{Format, Tree, WorkspaceResolved};

pub fn render(tree: &Tree, workspace: Option<&WorkspaceResolved>, config: &Config) -> String {
    let text = text::render_text(tree, workspace, config);
    let mut output = match config.format {
        Format::Text => text,
        Format::Md => md::render_md(&text),
    };
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

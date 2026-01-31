use std::fs;
use std::io::Read;
use std::path::Path;

use crate::model::ModuleKind;

pub fn read_summary(module_path: &Path, kind: ModuleKind) -> Option<String> {
    let summary = match kind {
        ModuleKind::Node => read_package_json(module_path),
        ModuleKind::Rust => read_cargo_toml(module_path),
        ModuleKind::Python => read_pyproject(module_path).or_else(|| read_setup_cfg(module_path)),
        ModuleKind::Go => read_go_mod(module_path),
        _ => None,
    };
    summary.or_else(|| read_readme_line(module_path))
}

fn read_package_json(module_path: &Path) -> Option<String> {
    let path = module_path.join("package.json");
    let content = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let name = value.get("name").and_then(|v| v.as_str());
    let desc = value.get("description").and_then(|v| v.as_str());

    if let (Some(name), Some(desc)) = (name, desc) {
        return Some(format!("{name} - {desc}"));
    }
    if let Some(name) = name {
        return Some(name.to_string());
    }
    if let Some(desc) = desc {
        return Some(desc.to_string());
    }

    let main = value.get("main").and_then(|v| v.as_str());
    if let Some(main) = main {
        return Some(format!("main: {main}"));
    }

    let bin = value.get("bin");
    if let Some(bin) = bin {
        if let Some(bin_str) = bin.as_str() {
            return Some(format!("bin: {bin_str}"));
        }
        if let Some(obj) = bin.as_object() {
            if let Some((key, _)) = obj.iter().next() {
                return Some(format!("bin: {key}"));
            }
        }
    }

    None
}

fn read_cargo_toml(module_path: &Path) -> Option<String> {
    let path = module_path.join("Cargo.toml");
    let content = fs::read_to_string(path).ok()?;
    let value: toml::Value = toml::from_str(&content).ok()?;
    let package = value.get("package")?.as_table()?;
    let name = package.get("name").and_then(|v| v.as_str());
    let desc = package.get("description").and_then(|v| v.as_str());

    match (name, desc) {
        (Some(name), Some(desc)) => Some(format!("{name} - {desc}")),
        (Some(name), None) => Some(name.to_string()),
        (None, Some(desc)) => Some(desc.to_string()),
        _ => None,
    }
}

fn read_pyproject(module_path: &Path) -> Option<String> {
    let path = module_path.join("pyproject.toml");
    let content = fs::read_to_string(path).ok()?;
    let value: toml::Value = toml::from_str(&content).ok()?;

    let project = value.get("project").and_then(|v| v.as_table());
    if let Some(project) = project {
        let name = project.get("name").and_then(|v| v.as_str());
        let desc = project.get("description").and_then(|v| v.as_str());
        return match (name, desc) {
            (Some(name), Some(desc)) => Some(format!("{name} - {desc}")),
            (Some(name), None) => Some(name.to_string()),
            (None, Some(desc)) => Some(desc.to_string()),
            _ => None,
        };
    }

    let poetry = value
        .get("tool")
        .and_then(|v| v.get("poetry"))
        .and_then(|v| v.as_table());
    if let Some(poetry) = poetry {
        let name = poetry.get("name").and_then(|v| v.as_str());
        let desc = poetry.get("description").and_then(|v| v.as_str());
        return match (name, desc) {
            (Some(name), Some(desc)) => Some(format!("{name} - {desc}")),
            (Some(name), None) => Some(name.to_string()),
            (None, Some(desc)) => Some(desc.to_string()),
            _ => None,
        };
    }

    None
}

fn read_setup_cfg(module_path: &Path) -> Option<String> {
    let path = module_path.join("setup.cfg");
    let content = fs::read_to_string(path).ok()?;
    let mut in_metadata = false;
    let mut name: Option<String> = None;
    let mut desc: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_metadata = trimmed.eq_ignore_ascii_case("[metadata]");
            continue;
        }
        if !in_metadata
            || trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with(';')
        {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if key == "name" && !value.is_empty() {
                name = Some(value.to_string());
            } else if key == "description" && !value.is_empty() {
                desc = Some(value.to_string());
            }
        }
    }

    match (name, desc) {
        (Some(name), Some(desc)) => Some(format!("{name} - {desc}")),
        (Some(name), None) => Some(name),
        (None, Some(desc)) => Some(desc),
        _ => None,
    }
}

fn read_go_mod(module_path: &Path) -> Option<String> {
    let path = module_path.join("go.mod");
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("module ") {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn read_readme_line(module_path: &Path) -> Option<String> {
    let entries = fs::read_dir(module_path).ok()?;
    let mut readmes: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.to_lowercase().starts_with("readme") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    readmes.sort();
    for name in readmes {
        let path = module_path.join(&name);
        if let Some(line) = read_first_line(&path) {
            return Some(line);
        }
    }
    None
}

fn read_first_line(path: &Path) -> Option<String> {
    let mut file = fs::File::open(path).ok()?;
    let mut buffer = Vec::new();
    let _ = file.by_ref().take(4096).read_to_end(&mut buffer).ok()?;
    let content = String::from_utf8_lossy(&buffer);
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

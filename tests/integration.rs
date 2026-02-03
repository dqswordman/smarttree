use std::fs;
use std::path::PathBuf;

use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn read_expected(fixture: &str, file: &str) -> String {
    let path = fixture_path(fixture).join(file);
    fs::read_to_string(path).expect("expected file")
}

fn run_and_assert(fixture: &str, expected_file: &str, args: &[&str]) {
    let expected = read_expected(fixture, expected_file);
    let mut cmd = cargo_bin_cmd!("smarttree");
    cmd.arg(fixture_path(fixture));
    for arg in args {
        cmd.arg(arg);
    }
    cmd.assert().success().stdout(expected);
}

#[test]
fn node_simple_module_text() {
    run_and_assert(
        "node_simple",
        "expected_module_text.txt",
        &["--lens", "module", "--format", "text", "--ascii"],
    );
}

#[test]
fn node_simple_module_md() {
    run_and_assert(
        "node_simple",
        "expected_module_md.md",
        &["--lens", "module", "--format", "md", "--ascii"],
    );
}

#[test]
fn pnpm_monorepo_module_text() {
    run_and_assert(
        "pnpm_monorepo",
        "expected_module_text.txt",
        &["--lens", "module", "--format", "text", "--ascii"],
    );
}

#[test]
fn rust_workspace_module_text() {
    run_and_assert(
        "rust_workspace",
        "expected_module_text.txt",
        &["--lens", "module", "--format", "text", "--ascii"],
    );
}

#[test]
fn python_project_module_text() {
    run_and_assert(
        "python_project",
        "expected_module_text.txt",
        &["--lens", "module", "--format", "text", "--ascii"],
    );
}

#[test]
fn init_creates_default_config() {
    let dir = tempdir().expect("tempdir");
    let mut cmd = cargo_bin_cmd!("smarttree");
    cmd.current_dir(dir.path());
    cmd.arg("--init");
    cmd.assert().success();

    let config_path = dir.path().join(".smarttree.yaml");
    let content = fs::read_to_string(config_path).expect("config file");
    assert!(content.contains("lens: module"));
    assert!(content.contains("format: text"));
    assert!(content.contains("unicode: false"));
}

#[test]
fn init_respects_custom_config_path() {
    let dir = tempdir().expect("tempdir");
    let config_path = dir.path().join("configs").join("smarttree.yml");
    let mut cmd = cargo_bin_cmd!("smarttree");
    cmd.current_dir(dir.path());
    cmd.arg("--init");
    cmd.arg("--config");
    cmd.arg(&config_path);
    cmd.assert().success();

    let content = fs::read_to_string(config_path).expect("config file");
    assert!(content.contains("max_children: 200"));
}

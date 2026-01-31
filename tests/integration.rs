use std::fs;
use std::path::PathBuf;

use assert_cmd::cargo::cargo_bin_cmd;

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

# AGENTS

Project context for Codex-style agents.

## Repo purpose
Build **smarttree**, a project-aware tree CLI written in Rust.

## Key commands
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`

## Conventions
- Default tree output uses ASCII characters; `--unicode` opts in to Unicode.
- Config precedence: CLI > config file > built-in defaults.
- Fixtures live in `tests/fixtures/` with golden outputs in `tests/fixtures/**/expected_*.txt`.
- Example outputs for the README live in `examples/outputs/`.

## Release checklist
- Update `README.md` examples if output format changes.
- Run tests and ensure fixtures still match.

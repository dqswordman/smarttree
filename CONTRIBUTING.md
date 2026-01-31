# Contributing

Thanks for helping improve **smarttree**!

## Quick start

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Guidelines

- Keep output stable and deterministic.
- Prefer small, well-scoped changes with tests.
- Update README examples if output format changes.

## Fixtures

Integration test fixtures live in `tests/fixtures/`. If you add a fixture,
include a golden output file (`expected_*.txt`) and cover it in `tests/integration.rs`.

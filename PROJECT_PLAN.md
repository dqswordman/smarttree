# Project Plan: smarttree

Milestone A: CLI + traversal
- Clap CLI with core flags.
- Config loading and precedence.
- Ignore + gitignore support.
- Depth/max-items/max-children enforced.
- Lens=files + format=text rendering.

Milestone B: Module lens MVP
- Module marker detection.
- Module summaries (package metadata + README fallback).
- Module lens rendering with key dirs/files.
- Format=md renderer (code fence).

Milestone C: Workspace/monorepo
- Workspace detection for pnpm/npm/lerna/cargo/go/turbo/nx.
- Pattern-based package discovery + heuristic fallback.
- Workspace-aware module filtering and labeling.

Milestone D: Tests + OSS polish
- Fixtures + integration tests with golden outputs.
- README, LICENSE, CHANGELOG, CONTRIBUTING.
- CI workflow for fmt/clippy/test.

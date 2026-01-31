# smarttree

**A project-aware tree command that shows modules and boundaries, not just files.**

## Why

`tree` shows files. `smarttree` shows *projects*.

### Before

```text
my-repo/
|-- apps
|   |-- web
|   |   |-- node_modules
|   |   |-- src
|   |   `-- package.json
|   `-- api
|       |-- src
|       `-- package.json
`-- packages
    |-- ui
    |   |-- src
    |   `-- package.json
    `-- utils
        |-- src
        `-- package.json
```

### After

```text
my-repo/  [workspace: pnpm]
|-- apps/
|   |-- api/  [node]  @acme/api - API server
|   |   |-- src/
|   |   `-- package.json
|   `-- web/  [node]  @acme/web - Web app
|       |-- src/
|       `-- package.json
`-- packages/
    |-- ui/  [node]  @acme/ui - UI components
    |   |-- src/
    |   `-- package.json
    `-- utils/  [node]  @acme/utils - Utilities
        |-- src/
        `-- package.json
```

## Install

```bash
cargo install smarttree
```

## Quick start

```bash
smarttree
smarttree . --lens files --depth 3
smarttree . --format md --max-children 60
```

## What makes it smart

- Project-aware defaults: ignores `.git`, `node_modules`, `dist`, `target`, caches, and IDE folders.
- Module lens: shows package boundaries with tags and summaries.
- Markdown output: copy/paste into PRs or README files.
- Monorepo-aware: understands common workspaces and groups packages.

## Usage

```bash
smarttree [PATH]

Options:
  --lens <module|files>
  --format <text|md>
  --depth <N>
  --max-items <N>
  --max-children <N>
  --respect-gitignore | --no-respect-gitignore
  --ignore <PATTERN> (repeatable)
  --include <PATTERN> (repeatable)
  --hidden
  --unicode | --ascii
  --config <FILE>
  --no-config
```

## Config file

`smarttree` looks for `.smarttree.yaml` in the root directory by default.

```yaml
lens: module
format: text
depth: 4
max_items: 20000
max_children: 200
respect_gitignore: true
hidden: false
unicode: false

ignore:
  - ".git"
  - "node_modules"
  - "dist"
include:
  - "packages/**/package.json"

key_dirs:
  - "src"
  - "tests"
  - "docs"
```

Precedence: CLI args > config file > built-in defaults.

## Examples

See `examples/outputs/` for ready-to-copy outputs.

## Roadmap

- Additional lenses (entry/docs/test)
- Per-module dependency summary
- Richer language metadata extraction

## License

MIT

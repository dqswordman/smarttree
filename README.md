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

### Option 1: Download a prebuilt binary (recommended for beginners)

1. Open the Releases page: https://github.com/dqswordman/smarttree/releases
2. Download the archive for your OS.
3. Unzip it.

### Option 2: Install with Rust (advanced)

```bash
cargo install smarttree
```

## Beginner quick start (no command-line experience)

### Windows (PowerShell)

1. Open PowerShell (press the Windows key, type "PowerShell", press Enter).
2. Go to your project folder:

```bash
cd C:\path\to\your\project
```

3. Run smarttree (use the full path to the binary):

```bash
C:\path\to\smarttree\smarttree.exe
```

### macOS / Linux (Terminal)

1. Open Terminal.
2. Go to your project folder:

```bash
cd /path/to/your/project
```

3. Run smarttree (use the full path to the binary):

```bash
/path/to/smarttree/smarttree
```

If you see "permission denied", make the binary executable first:

```bash
chmod +x /path/to/smarttree/smarttree
```

### Expected output (example)

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

## Quick start (experienced users)

```bash
smarttree
smarttree . --lens files --depth 3
smarttree . --format md --max-children 60
smarttree --init
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
  --init
```

## Config file

`smarttree` looks for `.smarttree.yaml` in the root directory by default.
Run `smarttree --init` to create a default config file, or pass `--config <FILE>`
to choose a different location.

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
  - "build"
  - "out"
  - ".next"
  - ".turbo"
  - ".cache"
  - ".pnpm-store"
  - "coverage"
  - ".venv"
  - "venv"
  - "__pycache__"
  - ".pytest_cache"
  - ".mypy_cache"
  - ".ruff_cache"
  - "target"
  - ".idea"
  - ".vscode"
  - ".DS_Store"
  - "Thumbs.db"

include: []

key_dirs:
  - "src"
  - "tests"
  - "test"
  - "docs"
  - "examples"
  - "scripts"
  - "public"
  - "include"
  - "cmd"
  - "bin"
```

Precedence: CLI args > config file > built-in defaults.

## Examples

See `examples/outputs/` for ready-to-copy outputs.

## Troubleshooting

- "command not found": run the binary with a full path, or add it to your PATH.
- "permission denied" on macOS/Linux: run `chmod +x /path/to/smarttree`.
- Missing files: try `--hidden` or `--no-respect-gitignore`, or adjust `ignore`/`include`.
- Weird characters in output: use `--ascii` (default) or `--unicode`.
- Output too large: lower `--depth`, `--max-items`, or `--max-children`.

## Compatibility notes

- Default output uses ASCII tree characters; opt in to Unicode with `--unicode`.
- Works on Windows, macOS, and Linux. Use the release binary for your OS.
- For very large repositories, tune traversal limits to keep output snappy.

## Roadmap

- Additional lenses (entry/docs/test)
- Per-module dependency summary
- Richer language metadata extraction

## License

MIT

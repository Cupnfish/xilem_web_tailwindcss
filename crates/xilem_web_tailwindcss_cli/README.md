# xilem_web_tailwindcss_cli

TailwindCSS CLI helper for `xilem_web` projects. This crate auto-downloads the
TailwindCSS binary when missing, then builds or watches CSS output.

> The auto-download workflow is inspired by the Tailwind integration in dioxus.

## Installation

```bash
cargo install xilem_web_tailwindcss_cli
```

## Quick Start

```bash
# Initialize Tailwind in your project
xilem-web-tailwindcss init

# Build CSS once
xilem-web-tailwindcss build

# Watch for changes
xilem-web-tailwindcss watch
```

## Commands

### `init`

Initialize Tailwind CSS files in an existing xilem_web project:

```bash
xilem-web-tailwindcss init
```

This creates:
- `tailwind.css` - Tailwind input file with `@import "tailwindcss";`
- `tailwind.config.js` - Configuration with content paths for `.rs` files
- `assets/` directory with `.gitignore`

Use `--force` to overwrite existing files.

### `build`

Build Tailwind CSS once (minified by default):

```bash
xilem-web-tailwindcss build

# Without minification
xilem-web-tailwindcss build --no-minify
```

### `watch`

Watch for changes and rebuild automatically:

```bash
xilem-web-tailwindcss watch
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--manifest-path` | | Directory or Cargo.toml path |
| `--input` | `-i` | Input CSS file (default: `tailwind.css`) |
| `--output` | `-o` | Output CSS file (default: `assets/tailwind.css`) |
| `--version` | | Tailwind version tag (default: `latest`) |
| `--no-downloads` | | Require `tailwindcss` binary in PATH |

## Environment Variables

- `XILEM_TAILWIND_NO_DOWNLOADS=1` - Disable automatic binary downloads
- `RUST_LOG=xilem_web_tailwindcss=debug` - Enable debug logging

## Examples

```bash
# Custom input/output paths
xilem-web-tailwindcss build -i src/styles.css -o dist/styles.css

# Use specific Tailwind version
xilem-web-tailwindcss --version v4.1.5 build

# Use system-installed tailwindcss
xilem-web-tailwindcss --no-downloads build
```

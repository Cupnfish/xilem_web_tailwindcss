# Tailwind Basic Example

A complete example demonstrating xilem_web with Tailwind CSS v4 using the `tw!` macro.

## Prerequisites

- Rust 1.85+
- [trunk](https://trunkrs.dev/) - `cargo install trunk`
- [xilem-web-tailwindcss CLI](../../crates/xilem_web_tailwindcss_cli)

## Quick Start

### 1. Install the CLI tool

```bash
cargo install --path ../../crates/xilem_web_tailwindcss_cli
```

### 2. Initialize Tailwind (for new projects)

If starting from scratch, use the `init` command:

```bash
xilem-web-tailwindcss init
```

This creates:
- `tailwind.css` - Tailwind input file
- `tailwind.config.js` - Tailwind configuration
- `assets/` directory with `.gitignore`

### 3. Build Tailwind CSS

```bash
# One-time build
xilem-web-tailwindcss build

# Or watch for changes (run in a separate terminal)
xilem-web-tailwindcss watch
```

### 4. Run with trunk

```bash
trunk serve
```

Open http://localhost:8080 in your browser.

## Development Workflow

For the best development experience, run these in separate terminals:

```bash
# Terminal 1: Watch Tailwind CSS changes
xilem-web-tailwindcss watch

# Terminal 2: Serve with trunk (hot reload)
trunk serve
```

## Project Structure

```
tailwind_basic/
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration (optional)
├── src/
│   └── main.rs         # xilem_web application
├── index.html          # HTML entry point
├── tailwind.css        # Tailwind CSS input
├── tailwind.config.js  # Tailwind configuration
└── assets/
    └── tailwind.css    # Generated CSS (after build)
```

## CLI Commands

```bash
# Initialize Tailwind in a project
xilem-web-tailwindcss init

# Initialize with force overwrite
xilem-web-tailwindcss init --force

# Build CSS once
xilem-web-tailwindcss build

# Build without minification
xilem-web-tailwindcss build --no-minify

# Watch and rebuild on changes
xilem-web-tailwindcss watch

# Specify custom input/output paths
xilem-web-tailwindcss build -i src/styles.css -o dist/styles.css
```

## Using the `tw!` macro

The `tw!` macro provides a convenient way to write Tailwind classes:

```rust
use xilem_web_tailwindcss::tw;

// Multiple string literals (space-separated classes)
.class(tw!("p-4 text-sm", "bg-blue-500"))

// Conditional classes
.class(tw!(
    "base-class",
    if condition => "active-class",
    if !condition => "inactive-class"
))
```

## Notes

- The CLI automatically downloads the Tailwind CSS binary if not found
- Tailwind scans `.rs` files for class names via the `content` config
- Generated CSS is placed in `assets/tailwind.css` by default

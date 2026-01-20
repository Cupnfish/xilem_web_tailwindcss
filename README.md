# xilem_web_tailwindcss

English | [中文](README.zh-CN.md)

Standalone repo providing TailwindCSS v4 support for `xilem_web`, with two crates:

| Crate | Description |
|------|-------------|
| `xilem_web_tailwindcss` | Runtime helpers (`tw!` macro, class splitting) |
| `xilem_web_tailwindcss_cli` | CLI tool (auto-download/run TailwindCSS) |

> The CLI download/run flow is inspired by the TailwindCSS integration in Dioxus.

## Install

### Runtime crate

```toml
# Cargo.toml
[dependencies]
xilem_web_tailwindcss = { git = "https://github.com/Cupnfish/xilem_web_tailwindcss.git" }
```

Or use a local path:

```toml
[dependencies]
xilem_web_tailwindcss = { path = "../xilem_web_tailwindcss/crates/xilem_web_tailwindcss" }
```

### CLI tool

Install from Git:

```bash
cargo install --git https://github.com/Cupnfish/xilem_web_tailwindcss.git xilem_web_tailwindcss_cli
```

Install from a local path:

```bash
cargo install --path crates/xilem_web_tailwindcss_cli
```

## Quick Start

### 1. Initialize the project

```bash
xilem-web-tailwindcss init
```

This creates:
- `tailwind.css` - Tailwind input file
- `tailwind.config.js` - config file (scans `.rs` files)
- `assets/` directory

### 2. Add the dependency

```toml
[dependencies]
xilem_web_tailwindcss = { git = "https://github.com/Cupnfish/xilem_web_tailwindcss.git" }
```

### 3. Use the `tw!` macro

```rust
use xilem_web::elements::html::div;
use xilem_web::interfaces::Element as _;
use xilem_web_tailwindcss::tw;

fn view(active: bool) -> impl xilem_web::interfaces::Element<()> {
    div("Hello").class(tw!(
        "px-4 py-2 text-sm",
        if active => "bg-blue-600 text-white",
        if !active => "bg-gray-200 text-gray-900",
    ))
}
```

### 4. Build CSS

```bash
# One-off build
xilem-web-tailwindcss build

# Watch mode (for development)
xilem-web-tailwindcss watch
```

### 5. Run the project

```bash
trunk serve
```

For development, you can run both with a single command:

```bash
xilem-web-tailwindcss dev
```

## Features

- Tailwind v4 workflow (defaults to GitHub latest release)
- `tw!` macro splits whitespace into class tokens, returning `Vec<Cow<'static, str>>`
- CLI runs without Node and downloads the official `tailwindcss` binary
- Supports `init` / `build` / `watch` / `dev`

## CLI Commands

### `init` - Initialize

```bash
xilem-web-tailwindcss init
xilem-web-tailwindcss init --force
```

### `build` - Build

```bash
xilem-web-tailwindcss build
xilem-web-tailwindcss build --no-minify
```

### `watch` - Watch

```bash
xilem-web-tailwindcss watch
```

### `dev` - Watch + serve

```bash
xilem-web-tailwindcss dev
```

Common trunk options are available:

```bash
xilem-web-tailwindcss dev --port 8085 --open
xilem-web-tailwindcss dev --address 0.0.0.0 --watch src
```

### Common options

| Option | Short | Description |
|--------|-------|-------------|
| `--manifest-path` | | Directory or Cargo.toml path |
| `--input` | `-i` | Input file (default `tailwind.css`) |
| `--output` | `-o` | Output file (default `assets/tailwind.css`) |
| `--version` | | Tailwind version (e.g. `v4.1.6` or `latest`) |
| `--no-downloads` | | Only use tailwindcss from PATH |

Environment variable: `XILEM_TAILWIND_NO_DOWNLOADS=1` disables auto-downloads.

## Dev workflow

Use a single command:

```bash
xilem-web-tailwindcss dev
```

Or run these in two terminals:

```bash
# Terminal 1: watch Tailwind CSS
xilem-web-tailwindcss watch

# Terminal 2: trunk hot reload
trunk serve
```

## Tailwind v4 config example

`tailwind.css`:

```css
@import "tailwindcss";
```

`tailwind.config.js`:

```js
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
};
```

## Examples

- **Full example**: `examples/tailwind_basic`
  ```bash
  cd examples/tailwind_basic
  xilem-web-tailwindcss build
  trunk serve
  ```

- **Macro example**: `crates/xilem_web_tailwindcss/examples/tw_macro.rs`
  ```bash
  cargo run -p xilem_web_tailwindcss --example tw_macro
  ```

## License

Apache-2.0

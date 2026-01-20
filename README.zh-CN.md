# xilem_web_tailwindcss

[English](README.md) | 中文

为 xilem_web 提供 TailwindCSS v4 支持的独立仓库，包含两个 crate：

| Crate | 描述 |
|-------|------|
| `xilem_web_tailwindcss` | 运行时辅助库（`tw!` 宏、类名拆分） |
| `xilem_web_tailwindcss_cli` | CLI 工具（自动下载/运行 TailwindCSS） |

> CLI 的自动下载与运行逻辑借鉴了 dioxus 的 TailwindCSS 集成实现。

## 安装

### 运行时库

```toml
# Cargo.toml
[dependencies]
xilem_web_tailwindcss = { git = "https://github.com/Cupnfish/xilem_web_tailwindcss.git" }
```

或使用本地路径：

```toml
[dependencies]
xilem_web_tailwindcss = { path = "../xilem_web_tailwindcss/crates/xilem_web_tailwindcss" }
```

### CLI 工具

从 Git 安装：

```bash
cargo install --git https://github.com/Cupnfish/xilem_web_tailwindcss.git xilem_web_tailwindcss_cli
```

从本地路径安装：

```bash
cargo install --path crates/xilem_web_tailwindcss_cli
```

## 快速开始

### 1. 初始化项目

```bash
xilem-web-tailwindcss init
```

这会创建：
- `tailwind.css` - Tailwind 输入文件
- `tailwind.config.js` - 配置文件（扫描 `.rs` 文件）
- `assets/` 目录

### 2. 添加依赖

```toml
[dependencies]
xilem_web_tailwindcss = { git = "https://github.com/Cupnfish/xilem_web_tailwindcss.git" }
```

### 3. 使用 `tw!` 宏

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

### 4. 构建 CSS

```bash
# 一次性构建
xilem-web-tailwindcss build

# 监听模式（开发时使用）
xilem-web-tailwindcss watch
```

### 5. 运行项目

```bash
trunk serve
```

## 功能概览

- Tailwind v4 工作流（默认使用 GitHub latest release）
- `tw!` 宏将空格拆分为 class token，输出 `Vec<Cow<'static, str>>`
- CLI 无需 Node 依赖，自动下载官方 tailwindcss 二进制
- 支持 `init`/`build`/`watch` 三种模式

## CLI 命令

### `init` - 初始化

```bash
xilem-web-tailwindcss init          # 初始化 Tailwind 文件
xilem-web-tailwindcss init --force  # 强制覆盖已有文件
```

### `build` - 构建

```bash
xilem-web-tailwindcss build             # 构建（默认 minify）
xilem-web-tailwindcss build --no-minify # 不压缩
```

### `watch` - 监听

```bash
xilem-web-tailwindcss watch  # 监听文件变化并自动重建
```

### 通用选项

| 选项 | 短选项 | 说明 |
|------|--------|------|
| `--manifest-path` | | 目录或 Cargo.toml 路径 |
| `--input` | `-i` | 输入文件（默认 `tailwind.css`） |
| `--output` | `-o` | 输出文件（默认 `assets/tailwind.css`） |
| `--version` | | Tailwind 版本（如 `v4.1.6` 或 `latest`） |
| `--no-downloads` | | 只使用 PATH 中的 tailwindcss |

环境变量：`XILEM_TAILWIND_NO_DOWNLOADS=1` 禁用自动下载

## 开发工作流

推荐在两个终端中分别运行：

```bash
# 终端 1: 监听 Tailwind CSS 变化
xilem-web-tailwindcss watch

# 终端 2: trunk 热重载
trunk serve
```

## Tailwind v4 配置示例

`tailwind.css`：

```css
@import "tailwindcss";
```

`tailwind.config.js`：

```js
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
};
```

## 示例

- **完整示例**：`examples/tailwind_basic`
  ```bash
  cd examples/tailwind_basic
  xilem-web-tailwindcss build
  trunk serve
  ```

- **宏示例**：`crates/xilem_web_tailwindcss/examples/tw_macro.rs`
  ```bash
  cargo run -p xilem_web_tailwindcss --example tw_macro
  ```

## 许可证

Apache-2.0

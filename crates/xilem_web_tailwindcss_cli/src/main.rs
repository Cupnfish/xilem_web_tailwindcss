use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

mod tailwind;

use tailwind::{CliSettings, TailwindCli};

#[derive(Parser, Debug)]
#[command(name = "xilem-web-tailwindcss")]
#[command(about = "TailwindCSS helper for xilem_web projects")]
struct Cli {
    /// Path to Cargo.toml or project directory.
    #[arg(long)]
    manifest_path: Option<PathBuf>,

    /// Path to the tailwind input CSS file.
    #[arg(long, short = 'i')]
    input: Option<PathBuf>,

    /// Path to the generated tailwind output CSS file.
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// Tailwind version tag (e.g. v4.1.5) or shorthand (v4/latest).
    #[arg(long)]
    version: Option<String>,

    /// Prefer using an existing tailwindcss binary from PATH.
    #[arg(long)]
    no_downloads: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize Tailwind CSS files in an existing `xilem_web` project.
    Init {
        /// Overwrite existing files.
        #[arg(long)]
        force: bool,
    },
    /// Build Tailwind CSS once.
    Build {
        /// Disable CSS minification.
        #[arg(long)]
        no_minify: bool,
    },
    /// Watch inputs and rebuild on changes.
    Watch,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_tracing();

    if cli.no_downloads {
        CliSettings::set_prefer_no_downloads(true);
    }

    let manifest_dir = resolve_manifest_dir(cli.manifest_path)?;

    match cli.command {
        Command::Init { force } => init_tailwind(&manifest_dir, force),
        Command::Build { no_minify } => {
            let tailwind = resolve_tailwind(&manifest_dir, cli.input.as_ref(), cli.version)?;
            tailwind.run_once(&manifest_dir, cli.input, cli.output, !no_minify)
        }
        Command::Watch => {
            let tailwind = resolve_tailwind(&manifest_dir, cli.input.as_ref(), cli.version)?;
            tailwind.watch(&manifest_dir, cli.input, cli.output)
        }
    }
}

fn init_tailwind(manifest_dir: &Path, force: bool) -> Result<()> {
    use std::fs;
    use tracing::info;

    let tailwind_css = manifest_dir.join("tailwind.css");
    let tailwind_config = manifest_dir.join("tailwind.config.js");
    let assets_dir = manifest_dir.join("assets");

    // Create tailwind.css
    if tailwind_css.exists() && !force {
        info!("tailwind.css already exists, skipping (use --force to overwrite)");
    } else {
        fs::write(&tailwind_css, TAILWIND_CSS_TEMPLATE)?;
        info!("Created tailwind.css");
    }

    // Create tailwind.config.js
    if tailwind_config.exists() && !force {
        info!("tailwind.config.js already exists, skipping (use --force to overwrite)");
    } else {
        fs::write(&tailwind_config, TAILWIND_CONFIG_TEMPLATE)?;
        info!("Created tailwind.config.js");
    }

    // Create assets directory
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir)?;
        info!("Created assets/ directory");
    }

    // Create .gitignore for assets if not exists
    let assets_gitignore = assets_dir.join(".gitignore");
    if !assets_gitignore.exists() {
        fs::write(&assets_gitignore, "tailwind.css\n")?;
        info!("Created assets/.gitignore");
    }

    info!("Tailwind CSS initialized successfully!");
    info!("");
    info!("Next steps:");
    info!("  1. Add xilem_web_tailwindcss to your Cargo.toml:");
    info!("     xilem_web_tailwindcss = \"0.1\"");
    info!("");
    info!("  2. Use the tw! macro in your code:");
    info!("     use xilem_web_tailwindcss::tw;");
    info!("     .class(tw!(\"p-4 text-sm bg-blue-500\"))");
    info!("");
    info!("  3. Build CSS: xilem-web-tailwindcss build");
    info!("  4. Or watch:  xilem-web-tailwindcss watch");

    Ok(())
}

const TAILWIND_CSS_TEMPLATE: &str = r#"@import "tailwindcss";
"#;

const TAILWIND_CONFIG_TEMPLATE: &str = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
};
"#;

fn resolve_tailwind(
    manifest_dir: &Path,
    input_path: Option<&PathBuf>,
    version: Option<String>,
) -> Result<TailwindCli> {
    if let Some(version) = version {
        let version = match version.as_str() {
            "v4" | "4" | "latest" => TailwindCli::LATEST_TAG.to_string(),
            _ => version,
        };
        return Ok(TailwindCli::new(version));
    }

    TailwindCli::autodetect(manifest_dir, input_path).ok_or_else(|| {
        anyhow!("unable to detect tailwind input; expected tailwind.css or --input. Run 'xilem-web-tailwindcss init' first.")
    })
}

fn resolve_manifest_dir(manifest_path: Option<PathBuf>) -> Result<PathBuf> {
    let path = manifest_path.unwrap_or_else(|| PathBuf::from("."));
    if path.is_dir() {
        return Ok(path);
    }
    let parent = path
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("manifest path has no parent directory"))?;
    Ok(parent)
}

fn init_tracing() {
    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "xilem_web_tailwindcss=info".to_string());
    let filter = EnvFilter::try_new(filter).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

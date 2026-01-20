use anyhow::{Context, Result, anyhow};
use clap::{Args, Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command as ProcessCommand, Stdio};
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod tailwind;

use tailwind::{CliSettings, TailwindCli};

#[derive(Parser, Debug)]
#[command(name = "xilem-web-tailwindcss")]
#[command(about = "TailwindCSS helper for xilem_web projects")]
struct Cli {
    /// Path to Cargo.toml or project directory.
    #[arg(long, global = true)]
    manifest_path: Option<PathBuf>,

    /// Path to the tailwind input CSS file.
    #[arg(long, short = 'i', global = true)]
    input: Option<PathBuf>,

    /// Path to the generated tailwind output CSS file.
    #[arg(long, short = 'o', global = true)]
    output: Option<PathBuf>,

    /// Tailwind version tag (e.g. v4.1.5) or shorthand (v4/latest).
    #[arg(long, global = true)]
    version: Option<String>,

    /// Prefer using an existing tailwindcss binary from PATH.
    #[arg(long, global = true)]
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
    /// Run Tailwind watch and `trunk serve` together.
    Dev {
        #[command(flatten)]
        trunk: TrunkServeOptions,
    },
}

#[derive(Args, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
struct TrunkServeOptions {
    /// Path to the Trunk config file.
    #[arg(long)]
    config: Option<PathBuf>,

    /// The addresses to serve on.
    #[arg(long, short = 'a')]
    address: Vec<String>,

    /// The port to serve on.
    #[arg(long, short = 'p')]
    port: Option<u16>,

    /// Open a browser tab once the initial build is complete.
    #[arg(long)]
    open: bool,

    /// Disable auto-reload of the web app.
    #[arg(long)]
    no_autoreload: bool,

    /// Disable fallback to index.html for missing files.
    #[arg(long)]
    no_spa: bool,

    /// Watch specific file(s) or folder(s).
    #[arg(long, short = 'w')]
    watch: Vec<PathBuf>,

    /// Paths to ignore.
    #[arg(long)]
    ignore: Vec<PathBuf>,

    /// The output dir for all final assets.
    #[arg(long, short = 'd')]
    dist: Option<PathBuf>,

    /// Build in release mode.
    #[arg(long)]
    release: bool,

    /// The public URL from which assets are to be served.
    #[arg(long)]
    public_url: Option<String>,
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
        Command::Dev { trunk } => {
            let tailwind = resolve_tailwind(&manifest_dir, cli.input.as_ref(), cli.version)?;
            run_dev(&manifest_dir, &tailwind, cli.input, cli.output, &trunk)
        }
    }
}

fn init_tailwind(manifest_dir: &Path, force: bool) -> Result<()> {
    use std::fs;

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
    let dir = if path.is_dir() {
        path
    } else {
        path.parent()
            .map(PathBuf::from)
            .ok_or_else(|| anyhow!("manifest path has no parent directory"))?
    };
    std::fs::canonicalize(&dir)
        .with_context(|| format!("failed to resolve manifest path {}", dir.display()))
}

fn run_dev(
    manifest_dir: &Path,
    tailwind: &TailwindCli,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    trunk: &TrunkServeOptions,
) -> Result<()> {
    info!("Starting Tailwind watch and trunk serve...");
    tailwind.ensure_installed()?;

    let mut tailwind_child = tailwind.run_with_stdio(
        manifest_dir,
        input_path,
        output_path,
        true,
        false,
        Stdio::inherit(),
        Stdio::inherit(),
    )?;
    let mut trunk_child = spawn_trunk(manifest_dir, trunk)?;

    wait_for_dev_exit(&mut tailwind_child, &mut trunk_child)
}

fn spawn_trunk(manifest_dir: &Path, trunk: &TrunkServeOptions) -> Result<Child> {
    let mut cmd = ProcessCommand::new("trunk");
    cmd.arg("serve");

    if let Some(config) = trunk.config.as_ref() {
        cmd.arg("--config").arg(config);
    }
    for address in &trunk.address {
        cmd.arg("--address").arg(address);
    }
    if let Some(port) = trunk.port {
        cmd.arg("--port").arg(port.to_string());
    }
    if trunk.open {
        cmd.arg("--open");
    }
    if trunk.no_autoreload {
        cmd.arg("--no-autoreload");
    }
    if trunk.no_spa {
        cmd.arg("--no-spa");
    }
    for watch in &trunk.watch {
        cmd.arg("--watch").arg(watch);
    }
    for ignore in &trunk.ignore {
        cmd.arg("--ignore").arg(ignore);
    }
    if let Some(dist) = trunk.dist.as_ref() {
        cmd.arg("--dist").arg(dist);
    }
    if trunk.release {
        cmd.arg("--release");
    }
    if let Some(public_url) = trunk.public_url.as_ref() {
        cmd.arg("--public-url").arg(public_url);
    }

    if let Some(value) = env::var_os("NO_COLOR") {
        match value.to_string_lossy().as_ref() {
            "1" => {
                cmd.env("NO_COLOR", "true");
            }
            "0" => {
                cmd.env("NO_COLOR", "false");
            }
            _ => {}
        }
    }

    let child = cmd
        .current_dir(manifest_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("failed to spawn trunk serve")?;
    Ok(child)
}

fn wait_for_dev_exit(tailwind: &mut Child, trunk: &mut Child) -> Result<()> {
    loop {
        if let Some(status) = tailwind.try_wait()? {
            terminate_child("trunk", trunk);
            return exit_status("tailwindcss watch", status);
        }

        if let Some(status) = trunk.try_wait()? {
            terminate_child("tailwindcss watch", tailwind);
            return exit_status("trunk serve", status);
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}

fn terminate_child(label: &str, child: &mut Child) {
    if let Err(err) = child.kill() {
        warn!(error = %err, "Failed to terminate {label} process");
    }

    if let Err(err) = child.wait() {
        warn!(error = %err, "Failed to wait for {label} process");
    }
}

fn exit_status(label: &str, status: std::process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("{label} exited with status {status}"))
    }
}

fn init_tracing() {
    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "xilem_web_tailwindcss=info".to_string());
    let filter = EnvFilter::try_new(filter).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

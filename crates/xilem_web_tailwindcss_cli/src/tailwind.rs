use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicU8, Ordering};
use tracing::{debug, info, warn};

// Inspired by the Tailwind integration in dioxus.

static NO_DOWNLOADS_OVERRIDE: AtomicU8 = AtomicU8::new(2);

#[derive(Debug, Default)]
pub struct CliSettings;

impl CliSettings {
    pub fn set_prefer_no_downloads(value: bool) {
        NO_DOWNLOADS_OVERRIDE.store(u8::from(value), Ordering::Relaxed);
    }

    pub fn prefer_no_downloads() -> bool {
        match NO_DOWNLOADS_OVERRIDE.load(Ordering::Relaxed) {
            0 => false,
            1 => true,
            _ => env_flag("XILEM_TAILWIND_NO_DOWNLOADS"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Workspace;

impl Workspace {
    pub fn xilem_data_dir() -> Result<PathBuf> {
        let project_dirs = ProjectDirs::from("org", "linebender", "xilem")
            .ok_or_else(|| anyhow!("unable to determine xilem data directory"))?;
        Ok(project_dirs.data_dir().to_path_buf())
    }
}

#[derive(Debug, Clone)]
pub struct TailwindCli {
    version: String,
}

impl TailwindCli {
    pub const LATEST_TAG: &'static str = "latest";

    pub fn new(version: String) -> Self {
        Self { version }
    }

    pub fn latest() -> Self {
        Self::new(Self::LATEST_TAG.to_string())
    }

    /// Use the latest Tailwind release when a tailwind input file is present.
    pub fn autodetect(manifest_dir: &Path, input_path: Option<&PathBuf>) -> Option<Self> {
        let input_exists = input_path.map_or_else(
            || manifest_dir.join("tailwind.css").exists(),
            |p| resolve_path(manifest_dir, p).exists(),
        );
        input_exists.then(Self::latest)
    }

    pub fn run_once(
        &self,
        manifest_dir: &Path,
        input_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
        minify: bool,
    ) -> Result<()> {
        self.ensure_installed()?;
        let output = self.run_with_output(manifest_dir, input_path, output_path, minify)?;

        if !output.status.success() {
            return Err(anyhow!("tailwindcss failed with status {}", output.status));
        }

        if !output.stderr.is_empty() {
            warn!(
                "Warnings while running tailwind: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    pub fn watch(
        &self,
        manifest_dir: &Path,
        input_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
    ) -> Result<()> {
        self.ensure_installed()?;

        let mut proc = self.run(manifest_dir, input_path, output_path, true, false)?;
        let stdin = proc.stdin.take();
        let status = proc.wait()?;
        drop(stdin);

        if !status.success() {
            return Err(anyhow!("tailwindcss watch exited with status {status}"));
        }

        Ok(())
    }

    pub fn run(
        &self,
        manifest_dir: &Path,
        input_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
        watch: bool,
        minify: bool,
    ) -> Result<Child> {
        let binary_path = self.get_binary_path()?;
        let input_path = resolve_input(manifest_dir, input_path);
        let output_path = resolve_output(manifest_dir, output_path)?;

        debug!("Spawning tailwindcss@{} with args: {:?}", self.version, {
            let mut args = vec![
                binary_path.to_string_lossy().to_string(),
                "--input".to_string(),
                input_path.to_string_lossy().to_string(),
                "--output".to_string(),
                output_path.to_string_lossy().to_string(),
            ];
            if watch {
                args.push("--watch".to_string());
            }
            if minify {
                args.push("--minify".to_string());
            }
            args
        });

        let mut cmd = Command::new(binary_path);
        let proc = cmd
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .args(watch.then_some("--watch"))
            .args(minify.then_some("--minify"))
            .current_dir(manifest_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to spawn tailwindcss")?;

        Ok(proc)
    }

    pub fn run_with_output(
        &self,
        manifest_dir: &Path,
        input_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
        minify: bool,
    ) -> Result<Output> {
        let binary_path = self.get_binary_path()?;
        let input_path = resolve_input(manifest_dir, input_path);
        let output_path = resolve_output(manifest_dir, output_path)?;

        let output = Command::new(binary_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .args(minify.then_some("--minify"))
            .current_dir(manifest_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("failed to run tailwindcss")?;

        Ok(output)
    }

    pub fn get_binary_path(&self) -> Result<PathBuf> {
        if CliSettings::prefer_no_downloads() {
            which::which("tailwindcss")
                .with_context(|| format!("missing tailwindcss@{}", self.version))
        } else {
            let installed_name = self.installed_bin_name();
            let install_dir = Self::install_dir()?;
            Ok(install_dir.join(installed_name))
        }
    }

    pub fn ensure_installed(&self) -> Result<()> {
        if self.get_binary_path()?.exists() {
            return Ok(());
        }
        info!("Installing tailwindcss@{}", self.version);
        self.install_github()
    }

    fn installed_bin_name(&self) -> String {
        let mut name = format!("tailwindcss-{}", self.version);
        if cfg!(windows) {
            name = format!("{name}.exe");
        }
        name
    }

    fn install_github(&self) -> Result<()> {
        debug!(
            "Attempting to install tailwindcss@{} from GitHub",
            self.version
        );

        let url = self.git_install_url().ok_or_else(|| {
            anyhow!(
                "no available GitHub binary for tailwindcss@{}",
                self.version
            )
        })?;

        let response = reqwest::blocking::get(url)
            .context("failed to download tailwindcss")?
            .error_for_status()
            .context("tailwindcss download returned error status")?;

        let binary_path = self.get_binary_path()?;
        if let Some(parent) = binary_path.parent() {
            std::fs::create_dir_all(parent).context("failed to create tailwindcss directory")?;
        }

        let bytes = response
            .bytes()
            .context("failed to read tailwindcss body")?;
        std::fs::write(&binary_path, &bytes).context("failed to write tailwindcss binary")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = binary_path.metadata()?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&binary_path, perms)?;
        }

        Ok(())
    }

    fn downloaded_bin_name() -> Option<String> {
        let platform = match target_lexicon::HOST.operating_system {
            target_lexicon::OperatingSystem::Linux => "linux",
            target_lexicon::OperatingSystem::Darwin => "macos",
            target_lexicon::OperatingSystem::Windows => "windows",
            _ => return None,
        };

        let arch = match target_lexicon::HOST.architecture {
            target_lexicon::Architecture::X86_64 | target_lexicon::Architecture::Aarch64(_)
                if platform == "windows" =>
            {
                "x64.exe"
            }
            target_lexicon::Architecture::X86_64 => "x64",
            target_lexicon::Architecture::Aarch64(_) => "arm64",
            _ => return None,
        };

        Some(format!("tailwindcss-{platform}-{arch}"))
    }

    fn install_dir() -> Result<PathBuf> {
        Ok(Workspace::xilem_data_dir()?.join("tailwind"))
    }

    fn git_install_url(&self) -> Option<String> {
        let binary = Self::downloaded_bin_name()?;
        if self.version == Self::LATEST_TAG {
            return Some(format!(
                "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/{binary}"
            ));
        }
        Some(format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/{}/{}",
            self.version, binary
        ))
    }
}

fn resolve_input(manifest_dir: &Path, input_path: Option<PathBuf>) -> PathBuf {
    input_path
        .map_or_else(|| manifest_dir.join("tailwind.css"), |p| {
            resolve_path(manifest_dir, &p)
        })
}

fn resolve_output(manifest_dir: &Path, output_path: Option<PathBuf>) -> Result<PathBuf> {
    let output_path = output_path
        .map_or_else(
            || manifest_dir.join("assets").join("tailwind.css"),
            |p| resolve_path(manifest_dir, &p),
        );
    let parent = output_path
        .parent()
        .ok_or_else(|| anyhow!("tailwind output path has no parent"))?;
    if !parent.exists() {
        std::fs::create_dir_all(parent).context("failed to create tailwindcss output directory")?;
    }
    Ok(output_path)
}

fn resolve_path(manifest_dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        manifest_dir.join(path)
    }
}

fn env_flag(name: &str) -> bool {
    let Some(value) = env::var_os(name) else {
        return false;
    };
    matches!(
        value.to_string_lossy().as_ref(),
        "1" | "true" | "TRUE" | "yes" | "YES"
    )
}

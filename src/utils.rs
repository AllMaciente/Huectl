use anyhow::{Context, Result};
use colored::*;
use directories::ProjectDirs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("", "", "huectl").context("Cannot determine XDG directories")
}

pub fn config_dir() -> Result<PathBuf> {
    let dirs = project_dirs()?;
    let config = dirs.config_dir().to_path_buf();

    std::fs::create_dir_all(&config)
        .with_context(|| format!("Cannot create config dir: {}", config.display()))?;
    Ok(config)
}

pub fn cache_dir() -> Result<PathBuf> {
    let dirs = project_dirs()?;
    let cache = dirs.cache_dir().to_path_buf();

    std::fs::create_dir_all(&cache)
        .with_context(|| format!("Cannot create cache dir: {}", cache.display()))?;
    Ok(cache)
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    false
}

pub fn run_hooks(
    hooks_dir: &Path,
    theme_name: &str,
    ctx: &HashMap<String, String>,
) -> Result<usize> {
    let mut hooks: Vec<PathBuf> = Vec::new();

    for entry in std::fs::read_dir(hooks_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && is_executable(&path) {
            hooks.push(path);
        }
    }

    hooks.sort();

    let mut success_count = 0;
    for hook_path in &hooks {
        let hook_name = hook_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let mut cmd = Command::new(hook_path);
        cmd.arg(theme_name);
        for (k, v) in ctx {
            cmd.env(format!("HUECTL_{}", k), v);
        }

        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    success_count += 1;
                } else {
                    eprintln!(
                        "  {} Hook '{}' failed with exit code: {}",
                        "!".yellow(),
                        hook_name,
                        status.code().unwrap_or(-1)
                    );
                }
            }
            Err(e) => {
                eprintln!("  {} Hook '{}' error: {}", "!".yellow(), hook_name, e);
            }
        }
    }

    Ok(success_count)
}

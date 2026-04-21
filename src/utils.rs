use anyhow::{Result, Context};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::process::Command;

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("","","huectl").context("Cannot determine XDG directories")
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

pub fn run_hook(hook: &std::path::Path, theme_name: &str) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(hook)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        std::fs::set_permissions(hook, perms)?;
    }
    let status = Command::new(hook)
        .arg(theme_name)
        .status()
        .with_context(|| format!("Cannot run hook: {}", hook.display()))?;
    if !status.success() {
        anyhow::bail!("Hook exited with status: {}", status);
    }
    Ok(())
}
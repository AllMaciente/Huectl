use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

const WALLPAPER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff"];

fn is_wallpaper_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| WALLPAPER_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn find_theme_wallpaper(theme_name: &str, wp_dir: &Path) -> Option<PathBuf> {
    if !wp_dir.exists() {
        return None;
    }

    for ext in WALLPAPER_EXTENSIONS {
        let candidate = wp_dir.join(format!("{}.{}", theme_name, ext));
        if candidate.exists() {
            return Some(candidate);
        }
    }

    if let Ok(entries) = std::fs::read_dir(wp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(stem) = path.file_stem() {
                if stem.to_string_lossy().to_lowercase() == theme_name.to_lowercase()
                    && is_wallpaper_file(&path)
                {
                    return Some(path);
                }
            }
        }
    }

    None
}



pub fn list_theme_wallpapers(theme_name: &str, wp_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let theme_dir = wp_dir.join(theme_name);
    if !theme_dir.exists() || !theme_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut wallpapers = Vec::new();
    for entry in std::fs::read_dir(&theme_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && is_wallpaper_file(&path) {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            wallpapers.push((name, path));
        }
    }
    wallpapers.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(wallpapers)
}

pub fn get_default_wallpaper_from_dir(theme_name: &str, wp_dir: &Path) -> Option<PathBuf> {
    let wallpapers = list_theme_wallpapers(theme_name, wp_dir).ok()?;
    wallpapers.into_iter().next().map(|(_, path)| path)
}

pub fn apply_wallpaper(wp_path: &Path, cache_dir: &Path) -> Result<()> {
    let absolute = wp_path
        .canonicalize()
        .with_context(|| format!("Cannot resolve wallpaper path: {}", wp_path.display()))?;

    let cache_file = cache_dir.join("wallpaper");
    std::fs::write(&cache_file, absolute.to_string_lossy().as_bytes())
        .context("Cannot write wallpaper cache file")?;

    let setters: &[(&str, &[&str])] = &[
        ("awww", &["img"]),
        ("swaybg", &["--image"]),
        ("feh", &["--bg-scale"]),
        ("nitrogen", &["--set-scaled", "--save"]),
        (
            "gsettings",
            &["set", "org.gnome.desktop.background", "picture-uri"],
        ),
        ("xwallpaper", &["--zoom"]),
        ("hsetroot", &["-fill"]),
    ];

    let path_str = absolute.to_string_lossy().to_string();

    for (cmd, args) in setters {
        if command_exists(cmd) {
            let mut full_args: Vec<&str> = args.to_vec();
            full_args.push(&path_str);

            let _ = Command::new(cmd).args(&full_args).spawn();
            return Ok(());
        }
    }

    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

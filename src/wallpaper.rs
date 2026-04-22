use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::process::Command;

const WALLPAPER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff"];

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
                if stem.to_string_lossy().to_lowercase() == theme_name.to_lowercase() {
                    if let Some(ext) = path.extension() {
                        if WALLPAPER_EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn apply_wallpaper(wp_path: &Path, cache_dir: &Path) -> Result<()> {
    let absolute = wp_path.canonicalize()
        .with_context(|| format!("Cannot resolve wallpaper path: {}", wp_path.display()))?;

    let cache_file = cache_dir.join("wallpaper");
    std::fs::write(&cache_file, absolute.to_string_lossy().as_bytes())
        .context("Cannot write wallpaper cache file")?;

    let setters: &[(&str, &[&str])] = &[
        ("swww",      &["img"]),
        ("swaybg",    &["--image"]),
        ("feh",       &["--bg-scale"]),
        ("nitrogen",  &["--set-scaled", "--save"]),
        ("gsettings", &["set", "org.gnome.desktop.background", "picture-uri"]),
        ("xwallpaper",&["--zoom"]),
        ("hsetroot",  &["-fill"]),
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

use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use colored::*;

use crate::theme::{Theme, load_theme};
use crate::cache::{Cache, save_colors_json, save_current_theme};
use crate::template::process_templates;
use crate::wallpaper::{apply_wallpaper, find_theme_wallpaper};
use crate::utils::{config_dir, cache_dir, run_hook};

#[derive(Parser)]
#[command(
    name = "huectl",
    about = "A fast, modular theme manager for Linux",
    version = "0.1.0",
    long_about = "Huectl manages themes for Linux systems using manual theme definitions and a template engine.\nThemes are stored in ~/.config/huectl/themes/ and templates in ~/.config/huectl/templates/"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Apply {
        theme_name: String,
      
        #[arg(long)]
        no_wallpaper: bool,

        #[arg(long)]
        no_templates: bool,
    },

    List,

    Colors,

    Preview {
        theme_name: String,
    },

    Wallpaper {
        #[command(subcommand)]
        command: WallpaperCommands,
    },

    Current,
}

#[derive(Subcommand)]
pub enum WallpaperCommands {
    Get,

    Set {
        path: String,
    },
}

pub fn cmd_apply(theme_name: &str, no_wallpaper: bool, no_templates: bool) -> Result<()> {
    println!("{} Applying theme: {}", "::".blue().bold(), theme_name.cyan().bold());

    let theme = load_theme(theme_name)
        .with_context(|| format!("Theme '{}' not found", theme_name))?;

    println!("  {} Loaded theme '{}'", "✓".green(), theme.name);

    theme.validate()?;
    println!("  {} Colors validated", "✓".green());

    let cache = cache_dir()?;
    save_colors_json(&theme, &cache)?;
    save_current_theme(&theme, &cache)?;
    println!("  {} Cache updated", "✓".green());

    if !no_templates {
        let config = config_dir()?;
        let tpl_dir = config.join("templates");
        if tpl_dir.exists() {
            let count = process_templates(&theme, &tpl_dir, &cache)?;
            println!("  {} Processed {} template(s)", "✓".green(), count);
        } else {
            println!("  {} No templates directory found ({})", "~".yellow(), tpl_dir.display());
        }
    }

    if !no_wallpaper {
        let config = config_dir()?;
        let wp_dir = config.join("wallpapers");
        match find_theme_wallpaper(theme_name, &wp_dir) {
            Some(wp_path) => {
                apply_wallpaper(&wp_path, &cache)?;
                println!("  {} Wallpaper applied: {}", "✓".green(), wp_path.display());
            }
            None => {
                println!("  {} No wallpaper found for theme '{}'", "~".yellow(), theme_name);
            }
        }
    }

    let hook = config_dir()?.join("hooks").join("post_apply.sh");
    if hook.exists() {
        println!("  {} Running post-apply hook...", "→".blue());
        run_hook(&hook, theme_name)?;
        println!("  {} Hook completed", "✓".green());
    }

    println!("\n{} Theme '{}' applied successfully!", "✓".green().bold(), theme_name.cyan().bold());
    Ok(())
}

pub fn cmd_list() -> Result<()> {
    let config = config_dir()?;
    let themes_dir = config.join("themes");

    if !themes_dir.exists() {
        println!("{} No themes directory found at {}", "!".yellow(), themes_dir.display());
        println!("  Create themes at: {}", themes_dir.display());
        return Ok(());
    }

    let cache = cache_dir()?;
    let current = Cache::load_current(&cache).map(|c| c.name).unwrap_or_default();

    let mut themes: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(&themes_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == themes_dir {
            continue;
        }
        if let Some(ext) = path.extension() {
            if ext == "json" || ext == "toml" {
                if let Some(stem) = path.file_stem() {
                    themes.push(stem.to_string_lossy().to_string());
                }
            }
        }
    }

    if themes.is_empty() {
        println!("{} No themes found in {}", "!".yellow(), themes_dir.display());
        return Ok(());
    }

    themes.sort();
    println!("{}", "Available themes:".bold());
    for t in &themes {
        if t == &current {
            println!("  {} {} {}", "●".green(), t.cyan().bold(), "(active)".dimmed());
        } else {
            println!("  {} {}", "○".dimmed(), t);
        }
    }
    println!("\n{} theme(s) found", themes.len());
    Ok(())
}

pub fn cmd_colors() -> Result<()> {
    let cache = cache_dir()?;
    let current = Cache::load_current(&cache)
        .context("No active theme. Run 'huectl apply <theme>' first.")?;

    println!("{} Active theme: {}\n", "::".blue().bold(), current.name.cyan().bold());

    let colors = &current.colors;

    println!("{}", "System colors:".bold());
    print_color_block("background", &colors.background);
    print_color_block("foreground", &colors.foreground);
    print_color_block("cursor",     &colors.cursor);

    println!("\n{}", "ANSI palette:".bold());
    let ansi_colors = [
        ("color0",  &colors.color0),  ("color8",  &colors.color8),
        ("color1",  &colors.color1),  ("color9",  &colors.color9),
        ("color2",  &colors.color2),  ("color10", &colors.color10),
        ("color3",  &colors.color3),  ("color11", &colors.color11),
        ("color4",  &colors.color4),  ("color12", &colors.color12),
        ("color5",  &colors.color5),  ("color13", &colors.color13),
        ("color6",  &colors.color6),  ("color14", &colors.color14),
        ("color7",  &colors.color7),  ("color15", &colors.color15),
    ];

    for chunk in ansi_colors.chunks(2) {
        let (n1, c1) = chunk[0];
        let (n2, c2) = chunk[1];
        let block1 = format_color_swatch(c1);
        let block2 = format_color_swatch(c2);
        println!("  {} {:<10} {}   {} {:<10} {}", block1, n1, c1, block2, n2, c2);
    }

    Ok(())
}

pub fn cmd_preview(theme_name: &str) -> Result<()> {
    let theme = load_theme(theme_name)
        .with_context(|| format!("Theme '{}' not found", theme_name))?;

    println!("{} Preview: {}\n", "::".blue().bold(), theme.name.cyan().bold());

    let c = &theme.colors;
    println!("{}", "System colors:".bold());
    print_color_block("background", &c.background);
    print_color_block("foreground", &c.foreground);
    print_color_block("cursor",     &c.cursor);

    println!("\n{}", "ANSI palette:".bold());
    let pairs = [
        ("color0",  &c.color0,  "color8",  &c.color8),
        ("color1",  &c.color1,  "color9",  &c.color9),
        ("color2",  &c.color2,  "color10", &c.color10),
        ("color3",  &c.color3,  "color11", &c.color11),
        ("color4",  &c.color4,  "color12", &c.color12),
        ("color5",  &c.color5,  "color13", &c.color13),
        ("color6",  &c.color6,  "color14", &c.color14),
        ("color7",  &c.color7,  "color15", &c.color15),
    ];
    for (n1, c1, n2, c2) in &pairs {
        println!("  {} {:<10} {}   {} {:<10} {}",
            format_color_swatch(c1), n1, c1,
            format_color_swatch(c2), n2, c2);
    }

    println!("\n{} This is a preview only. Run 'huectl apply {}' to apply.", "→".blue(), theme_name);
    Ok(())
}

pub fn cmd_wallpaper_get() -> Result<()> {
    let cache = cache_dir()?;
    let wp_file = cache.join("wallpaper");

    if wp_file.exists() {
        let path = std::fs::read_to_string(&wp_file)?.trim().to_string();
        println!("{}", path);
    } else {
        println!("{} No wallpaper currently set", "!".yellow());
    }
    Ok(())
}

pub fn cmd_wallpaper_set(path: &str) -> Result<()> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        anyhow::bail!("File not found: {}", path);
    }
    let cache = cache_dir()?;
    apply_wallpaper(p, &cache)?;
    println!("{} Wallpaper set to: {}", "✓".green(), path);
    Ok(())
}

pub fn cmd_current() -> Result<()> {
    let cache = cache_dir()?;
    let current = Cache::load_current(&cache)
        .context("No active theme. Run 'huectl apply <theme>' first.")?;

    println!("{} Current theme: {}", "::".blue().bold(), current.name.cyan().bold());

    let wp_file = cache.join("wallpaper");
    if wp_file.exists() {
        let path = std::fs::read_to_string(&wp_file)?.trim().to_string();
        println!("  Wallpaper: {}", path.dimmed());
    }
    Ok(())
}

fn print_color_block(name: &str, hex: &str) {
    let swatch = format_color_swatch(hex);
    println!("  {} {:<12} {}", swatch, name, hex);
}

fn format_color_swatch(hex: &str) -> String {
    if let Some((r, g, b)) = parse_hex(hex) {
        format!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b)
    } else {
        "  ".to_string()
    }
}

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let h = hex.trim_start_matches('#');
    if h.len() == 6 {
        let r = u8::from_str_radix(&h[0..2], 16).ok()?;
        let g = u8::from_str_radix(&h[2..4], 16).ok()?;
        let b = u8::from_str_radix(&h[4..6], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

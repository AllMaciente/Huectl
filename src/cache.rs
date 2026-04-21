use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub name:   String,
    pub colors: crate::theme::ThemeColors,
}

impl Cache {
    pub fn load_current(cache_dir: &Path) -> Result<Cache> {
        let file = cache_dir.join("current_theme.json");
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("Cannot read cache: {}", file.display()))?;
        serde_json::from_str(&content).context("Cannot parse current_theme.json")
    }
}

pub fn save_colors_json(theme: &Theme, cache_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let map = theme.colors.to_map();
    let json = serde_json::to_string_pretty(&map).context("Cannot serialize colors")?;
    let path = cache_dir.join("colors.json");
    std::fs::write(&path, json)
        .with_context(|| format!("Cannot write {}", path.display()))?;
    Ok(())
}

pub fn save_current_theme(theme: &Theme, cache_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let cache = Cache {
        name:   theme.name.clone(),
        colors: theme.colors.clone(),
    };
    let json = serde_json::to_string_pretty(&cache).context("Cannot serialize theme")?;
    let path = cache_dir.join("current_theme.json");
    std::fs::write(&path, json)
        .with_context(|| format!("Cannot write {}", path.display()))?;
    Ok(())
}

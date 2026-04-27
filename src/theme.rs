use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::utils::config_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,

    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

impl ThemeColors {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("color0".into(), self.color0.clone());
        m.insert("color1".into(), self.color1.clone());
        m.insert("color2".into(), self.color2.clone());
        m.insert("color3".into(), self.color3.clone());
        m.insert("color4".into(), self.color4.clone());
        m.insert("color5".into(), self.color5.clone());
        m.insert("color6".into(), self.color6.clone());
        m.insert("color7".into(), self.color7.clone());
        m.insert("color8".into(), self.color8.clone());
        m.insert("color9".into(), self.color9.clone());
        m.insert("color10".into(), self.color10.clone());
        m.insert("color11".into(), self.color11.clone());
        m.insert("color12".into(), self.color12.clone());
        m.insert("color13".into(), self.color13.clone());
        m.insert("color14".into(), self.color14.clone());
        m.insert("color15".into(), self.color15.clone());
        m.insert("background".into(), self.background.clone());
        m.insert("foreground".into(), self.foreground.clone());
        m.insert("cursor".into(), self.cursor.clone());
        m
    }

    pub fn to_map_stripped(&self) -> HashMap<String, String> {
        self.to_map()
            .into_iter()
            .map(|(k, v)| {
                (
                    format!("{}_strip", k),
                    v.trim_start_matches('#').to_string(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
    #[serde(default)]
    pub custom: std::collections::HashMap<String, String>,
}

impl Theme {
    pub fn validate(&self) -> Result<()> {
        let re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
        let map = self.colors.to_map();
        for (key, val) in &map {
            if !re.is_match(val) {
                bail!("Invalid color for '{}': '{}' (expected #RRGGBB)", key, val);
            }
        }
        Ok(())
    }

    pub fn template_context(&self) -> HashMap<String, String> {
        let mut ctx = self.colors.to_map();
        ctx.extend(self.colors.to_map_stripped());
        ctx.insert("theme_name".into(), self.name.clone());
        for (k, v) in &self.custom {
            ctx.insert(format!("custom_{}", k), v.clone());
        }
        ctx
    }
}

pub fn load_theme(name: &str) -> Result<Theme> {
    let config = config_dir()?;
    let themes_dir = config.join("themes");

    let json_path = themes_dir.join(format!("{}.json", name));
    let toml_path = themes_dir.join(format!("{}.toml", name));

    if json_path.exists() {
        load_theme_from_path(&json_path)
    } else if toml_path.exists() {
        load_theme_from_path(&toml_path)
    } else {
        bail!(
            "Theme '{}' not found.\nSearched:\n  {}\n  {}",
            name,
            json_path.display(),
            toml_path.display()
        )
    }
}

pub fn load_theme_from_path(path: &Path) -> Result<Theme> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read theme file: {}", path.display()))?;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("json");

    let theme: Theme = match ext {
        "toml" => toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML theme: {}", path.display()))?,
        _ => serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON theme: {}", path.display()))?,
    };

    Ok(theme)
}

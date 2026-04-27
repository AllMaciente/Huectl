use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::theme::Theme;

pub fn process_templates(theme: &Theme, tpl_dir: &Path, out_dir: &Path) -> Result<usize> {
    std::fs::create_dir_all(out_dir)?;

    let ctx = theme.template_context();
    let mut count = 0;

    for entry in WalkDir::new(tpl_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "tpl" {
            continue;
        }

        match process_single_template(path, &ctx, out_dir) {
            Ok(out_path) => {
                println!(
                    "    → {} → {}",
                    path.file_name().unwrap().to_string_lossy(),
                    out_path.display()
                );
                count += 1;
            }
            Err(e) => {
                eprintln!("    ✗ Failed: {}: {}", path.display(), e);
            }
        }
    }

    Ok(count)
}

fn process_single_template(
    tpl_path: &Path,
    ctx: &HashMap<String, String>,
    out_dir: &Path,
) -> Result<PathBuf> {
    let content = std::fs::read_to_string(tpl_path)
        .with_context(|| format!("Cannot read template: {}", tpl_path.display()))?;

    let rendered = render_string(&content, ctx);

    let file_name = tpl_path.file_name().unwrap().to_string_lossy();
    let out_name = file_name.strip_suffix(".tpl").unwrap_or(&file_name);
    let out_path = out_dir.join(out_name);

    std::fs::write(&out_path, rendered)
        .with_context(|| format!("Cannot write output: {}", out_path.display()))?;

    Ok(out_path)
}

pub fn render_string(template: &str, ctx: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in ctx {
        let placeholder = format!("{{{{{}}}}}", key); // {{key}}
        result = result.replace(&placeholder, value);
    }
    result
}

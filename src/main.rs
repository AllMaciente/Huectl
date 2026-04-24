mod cli;
mod theme;
mod template;
mod wallpaper;
mod cache;
mod utils;

use anyhow::Result;
use cli::{Cli, Commands, WallpaperCommands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply { theme_name, no_wallpaper, no_templates } => {
            cli::cmd_apply(&theme_name, no_wallpaper, no_templates)?;
        }
        Commands::List => {
            cli::cmd_list()?;
        }
        Commands::Colors => {
            cli::cmd_colors()?;
        }
        Commands::Preview { theme_name } => {
            cli::cmd_preview(&theme_name)?;
        }
        Commands::Wallpaper { command } => match command {
            WallpaperCommands::Get => {
                cli::cmd_wallpaper_get()?;
            }
            WallpaperCommands::Set { path } => {
                cli::cmd_wallpaper_set(&path)?;
            }
        },
        Commands::Current => {
            cli::cmd_current()?;
        }
        Commands::Reload => {
            cli::cmd_reload()?;
        }
    }

    Ok(())
}

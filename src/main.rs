mod cache;
mod cli;
mod template;
mod theme;
mod utils;
mod wallpaper;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, WallpaperCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply {
            theme_name,
            no_wallpaper,
            no_templates,
        } => {
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
        Commands::Var { key } => {
            cli::cmd_var(&key)?;
        }
    }

    Ok(())
}

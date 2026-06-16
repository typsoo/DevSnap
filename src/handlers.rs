pub mod delete;
pub mod init;
pub mod restore;
pub mod save;

use crate::cli::Commands;
use anyhow::{Context, Result};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn handle_command(command: Commands) {
    match command {
        Commands::Init {} => {
            if let Err(e) = init::handle_init() {
                eprintln!("Error saving workspace: {}", e);
            }
        }

        Commands::Save { name } => {
            if let Err(e) = save::handle_save(name) {
                eprintln!("Error saving workspace: {}", e);
            }
        }
        Commands::Restore {} => {
            if let Err(e) = restore::handle_restore() {
                eprintln!("Error restoring workspace: {}", e);
            }
        }
        Commands::Delete {} => {
            if let Err(e) = delete::handle_delete() {
                eprintln!("Error deleting workspace: {}", e);
            }
        }
        Commands::Show { name } => {
            println!(
                "Action: Delegate showing details for workspace '{}' to backend",
                name
            );
        }
        Commands::Close { name } => {
            println!("Action: Close selected app: '{}'", name);
        }
    }
}

fn select_workspace(action_name: &str) -> Result<Option<String>> {
    let workspaces = crate::storage_operations::list_snapshots()?;

    if workspaces.is_empty() {
        println!("No saved workspaces found. Nothing to {}.", action_name);
        return Ok(None);
    }

    let Some(index) = choose_workspace(&workspaces, &action_name)? else {
        println!("Operation cancelled.");
        return Ok(None);
    };

    Ok(Some(workspaces[index].clone()))
}

fn choose_workspace(workspaces: &[String], action_name: &str) -> Result<Option<usize>> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Select a workspace to {} (Use arrow keys and Enter, Esc to cancel)",
            action_name.to_uppercase()
        ))
        .default(0)
        .items(&workspaces)
        .interact_opt()
        .context("Error with elements selecting")
}

pub mod delete;
pub mod init;
pub mod restore;
pub mod rewrite;
pub mod save;

use crate::cli::Commands;
use anyhow::Result;
use inquire::{InquireError, Select};

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

        Commands::Rewrite {} => {
            if let Err(e) = rewrite::handle_rewrite() {
                eprintln!("Error rewriting workspace: {}", e);
            }
        }
    }
}

pub fn select_workspace(action_name: &str) -> Result<Option<String>> {
    let workspaces = crate::storage_operations::list_snapshots()?;

    if workspaces.is_empty() {
        println!("No saved workspaces found. Nothing to {}.", action_name);
        return Ok(None);
    }

    let prompt = format!(
        "Select a workspace to {} (Esc to cancel)",
        action_name.to_uppercase()
    );

    let selection = Select::new(&prompt, workspaces).prompt();

    match selection {
        Ok(choice) => Ok(Some(choice)),
        Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
            println!("Operation cancelled.");
            Ok(None)
        }
        Err(e) => Err(anyhow::anyhow!("Error with elements selecting: {}", e)),
    }
}

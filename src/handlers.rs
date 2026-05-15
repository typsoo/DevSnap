pub mod delete;
pub mod restore;
pub mod save;

use crate::cli::Commands;

pub fn handle_command(command: Commands) {
    match command {
        Commands::Save { name } => {
            if let Err(e) = save::handle_save(name) {
                eprintln!("Error saving workspace: {}", e);
            }
        }
        Commands::Restore { name } => {
            if let Err(e) = restore::handle_restore(name) {
                eprintln!("Error restoring workspace: {}", e);
            }
        }
        Commands::List => {
            println!("Action: Delegate listing workspaces to backend");
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
    }
}

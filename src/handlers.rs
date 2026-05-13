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
            println!("Action: Delegate restoring workspace '{}' to backend", name);
        }
        Commands::List => {
            println!("Action: Delegate listing workspaces to backend");
        }
        Commands::Delete { name } => {
            println!("Action: Delegate deleting workspace '{}' to backend", name);
        }
        Commands::Show { name } => {
            println!(
                "Action: Delegate showing details for workspace '{}' to backend",
                name
            );
        }
    }
}

use crate::apps_state_manager;
use crate::cli::Commands;
use crate::data_model::Workspace;
use crate::storage_creator;

pub fn handle_command(command: Commands) {
    match command {
        Commands::Save { name } => {
            let applications = apps_state_manager::collect_all_states();

            if applications.is_empty() {
                println!("No running applications detected to save.");
                return;
            }

            let workspace = Workspace {
                name: name.clone(),
                applications,
            };

            match storage_creator::save_snapshot(&workspace) {
                Ok(_) => println!("Workspace '{}' saved successfully.", name),
                Err(e) => eprintln!("Failed to save workspace: {}", e),
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

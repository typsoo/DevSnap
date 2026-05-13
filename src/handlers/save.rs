use crate::apps_state_manager;
use crate::data_model::Workspace;
use crate::storage_creator;
use anyhow::Result;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

pub fn handle_save(name: String) -> Result<()> {
    let mut applications = apps_state_manager::collect_all_states();

    if applications.is_empty() {
        println!("No running applications detected to save.");
        return Ok(());
    }

    if applications.len() > 1 {
        let items: Vec<String> = applications
            .iter()
            .map(|app| app.title.clone().unwrap_or_else(|| app.command.clone()))
            .collect();

        let chosen = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select windows to save (Space to select, Enter to confirm)")
            .items(&items)
            .interact()?;

        if chosen.is_empty() {
            println!("No windows selected. Save cancelled.");
            return Ok(());
        }

        applications = chosen
            .into_iter()
            .map(|i| applications[i].clone())
            .collect();
    }

    let count = applications.len();
    let workspace = Workspace {
        name: name.clone(),
        applications,
    };

    storage_creator::save_snapshot(&workspace)?;
    println!("'{}' saved successfully.", count);

    Ok(())
}

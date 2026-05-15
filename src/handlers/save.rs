use crate::apps_state_manager;
use crate::data_model::Workspace;
use crate::storage_operations::storage_creator;
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
            .map(|app| app.title.as_deref().unwrap_or(&app.command).to_string())
            .collect();

        let Some(chosen) = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select windows to save (Space to select, Enter to confirm)")
            .items(&items)
            .interact_opt()?
        else {
            println!("Selection cancelled.");
            return Ok(());
        };

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

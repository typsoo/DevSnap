use crate::apps_state_manager;
use crate::data_model::Workspace;
use crate::storage_operations::storage_creator;
use anyhow::Result;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

pub fn handle_save(name: String) -> Result<()> {
    let grouped_apps = apps_state_manager::collect_all_states();

    if grouped_apps.is_empty() {
        println!("No running applications detected to save.");
        return Ok(());
    }

    let app_names: Vec<String> = grouped_apps
        .iter()
        .map(|(app_name, _)| app_name.clone())
        .collect();

    let Some(chosen_app_indices) = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Step 1: Select applications to save (Space to select, Enter to confirm, Esc to quit)",
        )
        .items(&app_names)
        .interact_opt()?
    else {
        println!("Selection cancelled.");
        return Ok(());
    };

    if chosen_app_indices.is_empty() {
        println!("No applications selected. Aborting save.");
        return Ok(());
    }

    let mut final_applications = Vec::new();

    // Step 2: Iterate only through the applications the user selected
    for app_idx in chosen_app_indices {
        let (app_name, windows) = &grouped_apps[app_idx];

        // UX Improvement: If the app only has one active window, auto-select it
        // to save the user from clicking through an unnecessary one-item menu.
        if windows.len() == 1 {
            println!("  -> Auto-selecting the only open window for {}", app_name);
            final_applications.push(windows[0].clone());
            continue;
        }

        // Map windows to their display titles
        let window_titles: Vec<String> = windows
            .iter()
            .map(|app| app.title.as_deref().unwrap_or(&app.command).to_string())
            .collect();

        let prompt = format!(
            "Step 2: Select specific windows for {} (Space to select)",
            app_name
        );

        let Some(chosen_window_indices) = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt)
            .items(&window_titles)
            .interact_opt()?
        else {
            println!("Selection cancelled for {}.", app_name);
            continue;
        };

        // Collect the specifically chosen windows
        for win_idx in chosen_window_indices {
            final_applications.push(windows[win_idx].clone());
        }
    }

    // Final safety check before saving
    if final_applications.is_empty() {
        println!("No windows were selected. Aborting save.");
        return Ok(());
    }

    let count = final_applications.len();
    let workspace = Workspace {
        name: name.clone(),
        applications: final_applications,
    };

    // Save to disk
    storage_creator::save_snapshot(&workspace)?;
    println!(
        "Successfully saved {} window(s) to workspace '{}'.",
        count, name
    );

    Ok(())
}

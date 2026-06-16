use crate::apps_state_manager;
use crate::data_model::{Application, Workspace};
use crate::storage_operations;
use crate::storage_operations::storage_creator;
use anyhow::Result;
use inquire::{Confirm, MultiSelect};
use std::fmt;

#[derive(Clone)]
struct SelectableApp {
    index: usize,
    title: String,
}

impl fmt::Display for SelectableApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

pub fn handle_save(name: String) -> Result<()> {
    if !confirm_overwrite(&name)? {
        return Ok(());
    }

    let grouped_apps = apps_state_manager::collect_all_states();

    if grouped_apps.is_empty() {
        println!("No running applications detected to save.");
        return Ok(());
    }

    let chosen_app_names = select_applications(&grouped_apps)?;

    if chosen_app_names.is_empty() {
        println!("No applications selected. Aborting save.");
        return Ok(());
    }

    let mut final_applications = Vec::new();

    for app_name in chosen_app_names {
        let windows = grouped_apps
            .iter()
            .find(|(name, _)| name == &app_name)
            .map(|(_, windows)| windows)
            .expect("Selected app must exist in the grouped apps list");

        let mut selected_windows = select_windows_for_app(&app_name, windows)?;
        final_applications.append(&mut selected_windows);
    }

    if final_applications.is_empty() {
        println!("No windows were selected. Aborting save.");
        return Ok(());
    }

    let count = final_applications.len();
    let workspace = Workspace {
        name: name.clone(),
        applications: final_applications,
    };

    storage_creator::save_snapshot(&workspace)?;
    println!(
        "Successfully saved {} window(s) to workspace '{}'.",
        count, name
    );

    Ok(())
}

fn confirm_overwrite(name: &str) -> Result<bool> {
    let existing_workspaces = storage_operations::list_snapshots()?;

    if !existing_workspaces.iter().any(|ws| ws == name) {
        return Ok(true);
    }

    let prompt_text = format!(
        "Workspace '{}' already exists. Do you want to overwrite it?",
        name
    );

    match Confirm::new(&prompt_text).with_default(false).prompt() {
        Ok(true) => Ok(true),
        Ok(false) => {
            println!("Save operation cancelled. The existing workspace was kept safe.");
            Ok(false)
        }
        Err(_) => {
            println!("Operation cancelled.");
            Ok(false)
        }
    }
}

fn select_applications(grouped_apps: &[(String, Vec<Application>)]) -> Result<Vec<String>> {
    let app_names: Vec<String> = grouped_apps
        .iter()
        .map(|(app_name, _)| app_name.clone())
        .collect();

    let prompt =
        "Step 1: Select applications to save (Space to select, Enter to confirm, Esc to cancel)";

    match MultiSelect::new(prompt, app_names).prompt() {
        Ok(names) => Ok(names),
        Err(_) => {
            println!("Selection cancelled.");
            Ok(Vec::new())
        }
    }
}

fn select_windows_for_app(app_name: &str, windows: &[Application]) -> Result<Vec<Application>> {
    if windows.len() == 1 {
        println!("  -> Auto-selecting the only open window for {}", app_name);
        return Ok(vec![windows[0].clone()]);
    }

    let selectable_windows: Vec<SelectableApp> = windows
        .iter()
        .enumerate()
        .map(|(idx, app)| SelectableApp {
            index: idx,
            title: app.title.as_deref().unwrap_or(&app.command).to_string(),
        })
        .collect();

    let prompt = format!("Step 2: Select specific windows for {}", app_name);

    match MultiSelect::new(&prompt, selectable_windows).prompt() {
        Ok(chosen_items) => {
            if chosen_items.is_empty() {
                println!("Selection cancelled for {}.", app_name);
            }

            let selected_apps: Vec<Application> = chosen_items
                .into_iter()
                .map(|item| windows[item.index].clone())
                .collect();

            Ok(selected_apps)
        }
        Err(_) => {
            println!("Selection cancelled for {}.", app_name);
            Ok(Vec::new())
        }
    }
}

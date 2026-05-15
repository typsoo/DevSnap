use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn handle_delete() -> Result<()> {
    let workspaces = crate::storage_deleter::list_snapshots()?;

    if workspaces.is_empty() {
        println!("No saved workspaces found. Nothing to delete.");
        return Ok(());
    }

    let Some(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a workspace to DELETE (Use arrow keys and Enter, Esc to cancel)")
        .default(0)
        .items(&workspaces)
        .interact_opt()?
    else {
        println!("Deletion cancelled.");
        return Ok(());
    };

    crate::storage_deleter::delete_snapshot(&workspaces[index])?;
    println!(
        "Workspace '{}' has been successfully deleted.",
        &workspaces[index]
    );

    Ok(())
}

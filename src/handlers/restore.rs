use crate::storage_loader::load_snapshot;
use anyhow::{Result, anyhow};
use std::process::Command;

pub fn handle_restore(name: String) -> Result<()> {
    let workspace = load_snapshot(&name)?;

    if workspace.applications.is_empty() {
        println!("Workspace '{}' is empty. Nothing to restore.", name);
        return Ok(());
    }

    let failures: Vec<_> = workspace
        .applications
        .iter()
        .filter_map(|app| {
            Command::new(&app.command)
                .args(&app.args)
                .spawn()
                .err()
                .map(|e| format!("'{}': {}", app.command, e))
        })
        .collect();

    if failures.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("Failed to launch:\n{}", failures.join("\n")))
    }
}

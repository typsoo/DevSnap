use crate::data_model::Workspace;
use crate::storage_operations::get_snapshots_dir;
use anyhow::{Context, Result};
use std::fs;

pub fn load_snapshot(name: &str) -> Result<Workspace> {
    let dir = get_snapshots_dir()?;
    let file_path = dir.join(format!("{}.json", name));

    let data = fs::read_to_string(&file_path)
        .with_context(|| format!("Snapshot '{}' not found at {:?}", name, file_path))?;

    let workspace: Workspace = serde_json::from_str(&data).with_context(|| {
        format!(
            "Failed to parse snapshot '{}'. The file might be corrupted.",
            name
        )
    })?;

    Ok(workspace)
}

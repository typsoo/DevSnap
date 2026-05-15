use anyhow::{Context, Result};
use std::fs;

pub fn delete_snapshot(name: &str) -> Result<()> {
    let dir = crate::storage_operations::get_snapshots_dir()?;
    let file_path = dir.join(format!("{}.json", name));

    fs::remove_file(&file_path)
        .with_context(|| format!("Failed to delete snapshot file at {:?}", file_path))?;

    Ok(())
}

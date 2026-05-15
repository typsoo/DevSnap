use crate::storage_creator::get_snapshots_dir;
use anyhow::{Context, Result};
use std::fs;

pub fn list_snapshots() -> Result<Vec<String>> {
    let dir = get_snapshots_dir()?;
    let mut snapshots = Vec::new();

    for entry in fs::read_dir(dir).context("Failed to read snapshots directory")? {
        let entry = entry.context("Failed to read directory entry")?;

        let file_type = entry.file_type().context("Failed to get file type")?;
        if !file_type.is_file() {
            continue;
        }

        let path = entry.path();

        if !path.extension().is_some_and(|ext| ext == "json") {
            continue;
        }

        if let Some(file_stem) = path.file_stem().and_then(|name| name.to_str()) {
            snapshots.push(file_stem.to_string());
        }
    }

    Ok(snapshots)
}

pub fn delete_snapshot(name: &str) -> Result<()> {
    let dir = get_snapshots_dir()?;
    let file_path = dir.join(format!("{}.json", name));

    fs::remove_file(&file_path)
        .with_context(|| format!("Failed to delete snapshot file at {:?}", file_path))?;

    Ok(())
}

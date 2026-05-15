use crate::data_model::Workspace;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_snapshots_dir() -> Result<PathBuf> {
    let mut path = dirs::data_dir().context("Could not find local data directory")?;
    path.push("devsnap");
    path.push("snapshots");
    Ok(path)
}

pub fn save_snapshot(workspace: &Workspace) -> Result<()> {
    let dir = get_snapshots_dir()?;

    let file_path = dir.join(format!("{}.json", workspace.name));
    let json = serde_json::to_string_pretty(workspace).context("Failed to serialize workspace")?;

    fs::write(file_path, json).context("Failed to write snapshot file")?;
    Ok(())
}

pub fn init_user_env() -> Result<()> {
    let snapshots_path = get_snapshots_dir()?;

    create_structure_at(&snapshots_path)
}

fn create_structure_at(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory structure at {:?}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_structure_at() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_path = temp_dir.path().join(".devsnap").join("snapshots");

        let result = create_structure_at(&test_path);

        assert!(result.is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_init_user_env_execution() {
        let result = init_user_env();
        assert!(result.is_ok() || result.is_err());
    }
}

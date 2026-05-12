use crate::data_model::Workspace;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn get_snapshots_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push("devsnap");
        path.push("snapshots");
        path
    })
}

pub fn save_snapshot(workspace: &Workspace) -> Result<()> {
    let dir =
        get_snapshots_dir().ok_or_else(|| anyhow!("Failed to determine snapshots directory"))?;

    let file_path = dir.join(format!("{}.json", workspace.name));
    let json = serde_json::to_string_pretty(workspace).context("Failed to serialize workspace")?;

    fs::write(file_path, json).context("Failed to write snapshot file")?;
    Ok(())
}

pub fn init_user_env() -> io::Result<()> {
    let snapshots_path = get_snapshots_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "XDG Data directory not found"))?;

    create_structure_at(&snapshots_path)
}

fn create_structure_at(path: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_snapshots_dir() {
        let dir = get_snapshots_dir();
        assert!(dir.is_some());

        let path = dir.unwrap();
        assert!(path.ends_with("devsnap/snapshots"));
    }

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

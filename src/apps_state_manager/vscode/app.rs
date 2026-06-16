use crate::apps_state_manager::AppStateHandler;
use crate::data_model::Application;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub struct VSCodeStateHandler;

impl AppStateHandler for VSCodeStateHandler {
    fn target_app_name(&self) -> &'static str {
        "code"
    }

    fn capture_state(&self) -> Result<Option<Vec<Application>>> {
        let windows_dir = get_windows_dir()?;

        if !windows_dir.exists() {
            return Ok(None);
        }

        let entries =
            fs::read_dir(&windows_dir).context("Failed to read VS Code tracking directory")?;

        let apps: Vec<Application> = entries
            .map(|entry_result| -> Result<Option<Application>> {
                let path = entry_result
                    .context("Failed to read directory entry")?
                    .path();

                if !path.is_file() || !path.extension().is_some_and(|ext| ext == "json") {
                    return Ok(None);
                }

                let app = fs::read_to_string(&path)
                    .ok()
                    .and_then(|content| serde_json::from_str::<Value>(&content).ok())
                    .and_then(|json| window_to_application(&json));

                Ok(app)
            })
            .filter_map(|result| result.transpose())
            .collect::<Result<Vec<Application>>>()?;

        if apps.is_empty() {
            Ok(None)
        } else {
            Ok(Some(apps))
        }
    }
}

fn get_windows_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Could not determine user home directory")?;
    Ok(home_dir.join(".config/devsnap/vscode_windows"))
}

fn window_to_application(window: &Value) -> Option<Application> {
    let folder_path = window.get("folder").and_then(|f| f.as_str())?;

    Some(Application {
        command: "code".to_string(),
        args: vec![folder_path.to_string()],
        title: Some(format!("VS Code: {}", folder_path)),
        workspace_id: None,
    })
}

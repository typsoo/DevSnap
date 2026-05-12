use crate::apps_state_manager::AppStateHandler;
use crate::data_model::Application;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FirefoxSession {
    #[serde(default)]
    pub windows: Vec<Window>,
}

#[derive(Debug, Deserialize)]
pub struct Window {
    #[serde(default)]
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Deserialize)]
pub struct Tab {
    #[serde(default)]
    pub entries: Vec<Entry>,
    pub index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub url: String,
    pub title: Option<String>,
}

pub struct FirefoxStateHandler;

impl AppStateHandler for FirefoxStateHandler {
    fn target_app_name(&self) -> &'static str {
        "firefox"
    }

    fn capture_state(&self) -> Option<Vec<Application>> {
        let path = crate::apps_state_manager::session_files_finder::get_active_session_path()?;
        let bytes = std::fs::read(path).ok()?;

        if bytes.len() < 12 || &bytes[..8] != b"mozLz40\0" {
            return None;
        }

        let decompressed = lz4_flex::decompress_size_prepended(&bytes[8..]).ok()?;
        let session: FirefoxSession = serde_json::from_slice(&decompressed).ok()?;

        let urls: Vec<String> = session
            .windows
            .into_iter()
            .flat_map(|w| w.tabs)
            .filter_map(|t| {
                t.entries
                    .get(t.index.saturating_sub(1))
                    .map(|e| e.url.clone())
            })
            .collect();

        if urls.is_empty() {
            return None;
        }

        Some(vec![Application {
            command: "firefox".to_string(),
            args: urls,
            workspace_id: None,
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_app_name() {
        let handler = FirefoxStateHandler;
        assert_eq!(handler.target_app_name(), "firefox");
    }
}

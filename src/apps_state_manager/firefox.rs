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
    pub selected: usize,
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

        let apps: Vec<Application> = session
            .windows
            .into_iter()
            .filter_map(|w| {
                let tab_titles: Vec<String> = w
                    .tabs
                    .iter()
                    .take(3)
                    .filter_map(|t| t.entries.get(t.index.saturating_sub(1)))
                    .filter_map(|e| e.title.clone())
                    .collect();

                let window_title = if tab_titles.is_empty() {
                    "Firefox Window (Empty)".to_string()
                } else {
                    let mut title = tab_titles.join(", ");
                    if w.tabs.len() > 3 {
                        title.push_str(&format!(" (+{} more)", w.tabs.len() - 3));
                    }
                    title
                };

                let urls: Vec<String> = w
                    .tabs
                    .into_iter()
                    .filter_map(|t| {
                        t.entries
                            .get(t.index.saturating_sub(1))
                            .map(|e| e.url.clone())
                    })
                    .collect();

                if urls.is_empty() {
                    None
                } else {
                    Some(Application {
                        command: "firefox".to_string(),
                        args: urls,
                        title: Some(window_title),
                        workspace_id: None,
                    })
                }
            })
            .collect();

        if apps.is_empty() { None } else { Some(apps) }
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

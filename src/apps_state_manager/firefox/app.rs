use crate::apps_state_manager::AppStateHandler;
use crate::data_model::Application;
use anyhow::{Context, Result, bail};
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

const MOZLZ4: &[u8] = b"mozLz40\0";
const MAX_PREVIEW_TABS: usize = 3;

impl AppStateHandler for FirefoxStateHandler {
    fn target_app_name(&self) -> &'static str {
        "firefox"
    }

    fn capture_state(&self) -> Result<Option<Vec<Application>>> {
        let path =
            crate::apps_state_manager::firefox::session_files_finder::get_active_session_path()?;

        let session = read_session_file(&path)?;
        let apps: Vec<Application> = session
            .windows
            .into_iter()
            .filter_map(window_to_application)
            .collect();

        if apps.is_empty() {
            Ok(None)
        } else {
            Ok(Some(apps))
        }
    }
}

fn read_session_file(path: &std::path::Path) -> Result<FirefoxSession> {
    let bytes =
        std::fs::read(path).with_context(|| format!("Failed to read session file: {:?}", path))?;

    if !bytes.starts_with(MOZLZ4) {
        bail!("File missing expected MOZLZ4 header: {:?}", path);
    }

    let compressed = &bytes[MOZLZ4.len()..];
    let decompressed = lz4_flex::decompress_size_prepended(compressed)
        .context("Failed to decompress lz4_flex payload")?;
    serde_json::from_slice(&decompressed).context("Failed to parse Firefox session JSON")
}

fn window_to_application(window: Window) -> Option<Application> {
    let tabs_len = window.tabs.len();

    let mut urls = Vec::with_capacity(tabs_len);
    let mut preview_titles = Vec::with_capacity(MAX_PREVIEW_TABS);

    for tab in window.tabs {
        if let Some(entry) = tab
            .index
            .checked_sub(1)
            .and_then(|i| tab.entries.into_iter().nth(i))
        {
            urls.push(entry.url);

            if preview_titles.len() < MAX_PREVIEW_TABS {
                preview_titles.extend(entry.title);
            }
        }
    }

    if urls.is_empty() {
        return None;
    }

    Some(Application {
        command: "firefox".to_string(),
        args: urls,
        title: format_titles(preview_titles),
        workspace_id: None,
    })
}

fn format_titles(titles: Vec<String>) -> Option<String> {
    if titles.is_empty() {
        Some("Firefox Window (Empty)".to_string())
    } else {
        Some(titles.join("; "))
    }
}

#[cfg(test)]
mod tests {}

pub mod firefox;
pub mod vscode;

use crate::apps_state_manager::firefox::app::FirefoxStateHandler;
use crate::apps_state_manager::vscode::app::VSCodeStateHandler;

use crate::data_model::Application;
use anyhow::Result;

pub trait AppStateHandler {
    fn target_app_name(&self) -> &'static str;

    fn capture_state(&self) -> Result<Option<Vec<Application>>>;
}

pub fn collect_all_states() -> Vec<(String, Vec<Application>)> {
    let handlers: Vec<Box<dyn AppStateHandler>> =
        vec![Box::new(FirefoxStateHandler), Box::new(VSCodeStateHandler)];

    let mut grouped_applications = Vec::new();

    for handler in handlers {
        match handler.capture_state() {
            Ok(Some(apps)) if !apps.is_empty() => {
                grouped_applications.push((handler.target_app_name().to_string(), apps));
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Failed to capture state for {}: {}",
                    handler.target_app_name(),
                    e
                );
            }
        }
    }

    grouped_applications
}

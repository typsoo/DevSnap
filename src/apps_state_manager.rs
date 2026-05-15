pub mod firefox;
pub mod session_files_finder;

use crate::apps_state_manager::firefox::FirefoxStateHandler;
use crate::data_model::Application;
use anyhow::Result;

pub trait AppStateHandler {
    fn target_app_name(&self) -> &'static str;

    fn capture_state(&self) -> Result<Option<Vec<Application>>>;
}

pub fn collect_all_states() -> Vec<Application> {
    let handlers: Vec<Box<dyn AppStateHandler>> = vec![Box::new(FirefoxStateHandler)];

    let mut all_applications = Vec::new();

    for handler in handlers {
        match handler.capture_state() {
            Ok(Some(mut apps)) => {
                all_applications.append(&mut apps);
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!(
                    "Failed to capture state for {}: {}",
                    handler.target_app_name(),
                    e
                );
            }
        }
    }

    all_applications
}

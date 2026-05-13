pub mod firefox;
pub mod session_files_finder;

use crate::apps_state_manager::firefox::FirefoxStateHandler;
use crate::data_model::Application;

pub trait AppStateHandler {
    fn target_app_name(&self) -> &'static str;

    fn capture_state(&self) -> Option<Vec<Application>>;
}

//change it
pub fn collect_all_states() -> Vec<Application> {
    let handlers: Vec<Box<dyn AppStateHandler>> = vec![Box::new(FirefoxStateHandler)];

    handlers
        .into_iter()
        .filter_map(|h| h.capture_state())
        .flatten()
        .collect()
}

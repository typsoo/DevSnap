mod cli;
mod handlers;
mod storage_creator;
use crate::apps_state_manager::session_files_finder::{
    get_active_session_path, get_firefox_base_dir,
};

pub mod apps_state_manager;

use clap::Parser;
use cli::Cli;

fn main() {
    if let Err(e) = storage_creator::init_user_env() {
        eprintln!("Failed to initialize storage environment: {}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    handlers::handle_command(cli.command);
}

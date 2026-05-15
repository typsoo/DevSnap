pub mod apps_state_manager;
mod cli;
pub mod data_model;
mod handlers;
mod storage_creator;
pub mod storage_deleter;
pub mod storage_loader;

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

#[cfg(test)]
mod tests {}

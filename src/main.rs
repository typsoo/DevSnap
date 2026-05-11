mod cli;
mod env_manager;
mod handlers;
mod storage;

use clap::Parser;
use cli::Cli;

fn main() {
    if let Err(e) = storage::init_user_env() {
        eprintln!("Failed to initialize storage environment: {}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    handlers::handle_command(cli.command);
}

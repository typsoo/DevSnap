pub mod apps_state_manager;
mod cli;
pub mod data_model;
mod handlers;
mod storage_creator;

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
mod tests {
    use std::process::Command;

    #[test]
    fn test() {
        println!("Launching...");

        // Command::new принимает имя исполняемого файла (например, "firefox" или "code")
        let result = Command::new("firefox")
            // .args() принимает вектор строк (наши URL-адреса или пути к папкам)
            .arg("https://github.com/leonardomso/rust-skills")
            // .spawn() запускает процесс отвязанно от нашего CLI
            .status();

        match result {
            Ok(child) => {
                println!("Successfully launched firefox (PID: {})", child);
            }
            Err(e) => {
                eprintln!("Failed to launch: {}", e);
            }
        }
    }
}

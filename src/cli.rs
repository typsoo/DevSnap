use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "devsnap")]
#[command(about = "Developer Workspace Snapshot CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Save { name: String },
    Restore,
    Delete,
    Rewrite,
}

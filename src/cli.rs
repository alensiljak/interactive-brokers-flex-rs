/*
 * CLI definition
 */

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // #[command(about = "Configuration")]
    // #[clap(subcommand)]
    // Cfg(ConfigCmd),
    Cfg,
    Dl,
    Compare
}

#[derive(Debug, Subcommand)]
pub enum ConfigCmd {
    /// Shows the configuration file path
    // Path,
    Show,
}
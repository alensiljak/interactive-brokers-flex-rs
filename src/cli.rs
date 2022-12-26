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
    #[command(about = "display configuration settings")]
    Cfg,
    #[command(about = "downloads the IB Flex Cash Tx report")]
    Dl,
    #[command(about = "compares IB Flex Cash Tx report and Ledger")]
    Compare
}

// #[derive(Debug, Subcommand)]
// pub enum ConfigCmd {
//     /// Shows the configuration file path
//     // Path,
//     Show,
// }
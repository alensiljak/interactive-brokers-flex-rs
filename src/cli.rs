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
    Dl(DlParams),
    #[command(about = "compares IB Flex Cash Tx report and Ledger")]
    Compare
}

#[derive(Debug, clap::Args)]
pub struct DlParams {
    #[arg(short, long)]
    pub query_id: Option<u32>,
    pub token: Option<String>
}
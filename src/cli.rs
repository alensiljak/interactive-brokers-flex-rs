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
    #[command(about = "downloads the IB Flex Cash Tx report")]
    Dl(DlParams),
    #[command(about = "compares IB Flex Cash Tx report and Ledger")]
    Cmp(CmpParams)
}

#[derive(Debug, clap::Args)]
pub struct DlParams {
    #[arg(short, long)]
    pub query_id: Option<u32>,
    #[arg(short, long)]
    pub token: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct CmpParams {
    #[arg(short, long, help="The report .xml to use for comparison")]
    pub flex_report_path: Option<String>,
    #[arg(short='d', long, help="Directory that contains the Flex .xml reports")]
    pub flex_reports_dir: Option<String>,
    #[arg(short='j', long, help="Ledger journal file to use (.ledger)")]
    pub ledger_journal_file: Option<String>,
    #[arg(short, long, help="The symbols data file")]
    pub symbols_path: String,
    /// Use Ledger's effective dates
    #[arg(short, long)]
    pub effective: bool,
}
/*!
 * CLI for ibflex library
 */

use clap::Parser;
use cli::{Cli, Commands};
use ibflex::{compare::CompareParams, config::get_config, download::DownloadParams};

/*
 * CLI for operating the library
 */
mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dl(params) => {
            println!("downloading report...");

            let dl_params = DownloadParams::new(params.query_id, &params.token);
            let path = ibflex::download::download(dl_params).await;

            println!("Flex Query saved to {path}");
        }
        Commands::Cmp(params) => {
            let cmp_params =
                CompareParams::new(params.flex_report_path.to_owned(), params.ledger_init_file.to_owned());
            ibflex::compare::compare(cmp_params).expect("transactions compared");
        }
        Commands::Cfg => {
            let cfg = get_config(DownloadParams::default());
            println!("{:?}", cfg);
        }
    }
}

/*!
 * CLI for ibflex library
 */

use clap::Parser;
use cli::{Cli, Commands};
use ibflex::{download::DownloadParams, config::get_config};

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
        Commands::Compare => {
            ibflex::compare::compare();
        }
        Commands::Cfg => {
            let cfg = get_config(DownloadParams::default());
            println!("{:?}", cfg);
        }
    }
}

/*!
 * CLI for ibflex library
 */

use clap::Parser;
use cli::{Cli, Commands};
use ibflex::read_config;

/*
 * CLI for operating the library
 */
mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dl => {
            println!("downloading report...");

            let path = ibflex::download::download().await;

            println!("Flex Query saved to {path}");
        }
        Commands::Compare => {
            ibflex::compare::compare();
        }
        Commands::Cfg => {
            let cfg = read_config();
            println!("{:?}", cfg);
        }
    }
}

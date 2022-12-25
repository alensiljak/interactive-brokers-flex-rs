use clap::Parser;
use cli::{Cli, Commands};

/*
 * CLI for operating the library
 */
mod cli;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Dl => {
            log::debug!("downloading...");
            todo!("download prices");
        },
        Commands::Compare => {
            ibflex::compare::compare().await;
        }
    };

    Ok(())
}

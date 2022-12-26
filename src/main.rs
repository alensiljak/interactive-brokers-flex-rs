use clap::Parser;
use cli::{Cli, Commands};

/*
 * CLI for operating the library
 */
mod cli;

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Dl => {
            log::debug!("downloading...");
            todo!("download prices");
        },
        Commands::Compare => {
            ibflex::compare::compare();
        }
    };

}

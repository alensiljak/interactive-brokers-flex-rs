/*!
 * Interactive Brokers Flex library
 * 
 * Project documentation: <https://github.com/alensiljak/interactive-brokers-flex-rs>
 */
pub mod compare;
pub mod config;
pub mod download;
pub mod flex_query_def;
pub mod flex_query_parser;
pub mod flex_statement;

//pub const APP_NAME: &str = "ibflex";

/**
 * Reads the current configuration from the config file.
 * The config file is expected to be in the current directory and be named `ibflex.toml`.
 */
pub fn read_config() -> config::Config {
    // confy::load(APP_NAME, None)
    confy::load_path("./ibflex.toml").expect("configuration loaded")
}

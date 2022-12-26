/*
 * Shareable library
 */
pub mod compare;
pub mod config;
pub mod download;
pub mod flex_query_def;
pub mod flex_query_parser;
pub mod flex_statement;

pub const APP_NAME: &str = "ibflex";

pub fn read_config() -> config::Config {
    // confy::load(APP_NAME, None)
    confy::load_path("./ibflex.toml").expect("configuration loaded")
}

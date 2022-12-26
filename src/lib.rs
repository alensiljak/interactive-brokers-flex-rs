/*
 * Shareable library
 */
pub mod compare;
pub mod config;
pub mod download;
pub mod flex_report;
pub mod ib_flex_parser;

pub const APP_NAME: &str = "ibflex";

pub fn read_config() -> config::Config {
    // confy::load(APP_NAME, None)
    confy::load_path("./ibflex.toml").expect("configuration loaded")
}

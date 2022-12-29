/*!
 * The configuration file structure
 */

use serde::{Deserialize, Serialize};

use crate::download::DownloadParams;

/**
 * Configuration structure for ibflex
 */
#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    // Download
    pub flex_query_id: String,
    pub ib_token: String,
    // Comparison
    pub flex_report_path: Option<String>,
    pub flex_reports_dir: Option<String>,
    pub ledger_init_file: Option<String>,
}

/**
 * Collects the  configuration values in order of priority:
 * 1.) command-line parameters,
 * 2.) environment variables,
 * 3.) config file in the current directory
 */
pub fn get_config(params: DownloadParams) -> Config {
    let mut cfg = read_config_file();

    // overwrite the file values if provided by other means
    
    if let Some(query_id) = params.query_id {
        cfg.flex_query_id = query_id.to_string();
    } else if let Ok(env_queryid) = std::env::var("IBFLEX_QUERYID") {
        cfg.flex_query_id = env_queryid;
    };

    if let Some(token) = params.token {
        cfg.ib_token = token;
    } else if let Ok(env_tkn) = std::env::var("IBFLEX_TOKEN") {
        cfg.ib_token = env_tkn;
    };

    cfg
}

/**
 * Reads the current configuration from the config file.
 * The config file is expected to be in the current directory and be named `ibflex.toml`.
 */
pub fn read_config_file() -> Config {
    // confy::load(APP_NAME, None)

    let path = "./ibflex.toml";

    confy::load_path(path).expect("configuration loaded")
}

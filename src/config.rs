/*!
 * The configuration file structure
 */

use serde::{Deserialize, Serialize};

use crate::{download::DownloadParams, compare::CompareParams};

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
pub fn get_dl_config(params: DownloadParams) -> Config {
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

pub fn get_cmp_config(params: &CompareParams) -> Config {
    let mut cfg = read_config_file();

    if let Some(flex_report_path) = &params.flex_report_path {
        cfg.flex_report_path = Some(flex_report_path.to_owned());
    }
    if let Some(flex_reports_dir) = &params.flex_reports_dir {
        cfg.flex_reports_dir = Some(flex_reports_dir.to_owned());
    }
    if let Some(ledger_init_file) = &params.ledger_init_file {
        cfg.ledger_init_file = Some(ledger_init_file.to_owned());
    }

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

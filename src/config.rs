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
    pub flex_query_id: String,
    pub ib_token: String,
}

/**
 * Check all the places for the configuration value.
 * 1.) If the values have been passed as command-line parameters, keep these,
 * 2.) Read the environment variables,
 * 3.) Read the config file in the current directory
 */
pub fn get_config(params: DownloadParams) -> Config {
    Config {
        flex_query_id: if let Some(query_id) = params.query_id {
            query_id.to_string()
        } else if let Ok(env_queryid) = std::env::var("IBFLEX_QUERYID") {
            env_queryid
        } else {
            // read from the config file
            let cfg_file = read_config_file();
            cfg_file.flex_query_id
        },
        ib_token: if let Some(token) = params.token {
            token
        } else if let Ok(env_tkn) = std::env::var("IBFLEX_TOKEN") {
            env_tkn
        } else {
            // read from the config file
            let cfg_file = read_config_file();
            cfg_file.ib_token
        },
    }
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

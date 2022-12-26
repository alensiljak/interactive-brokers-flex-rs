use serde::{Deserialize, Serialize};

use crate::APP_NAME;

/**
 * Configuration structure for ibflex
 */
#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub flex_query_id: String,
    pub ib_token: String,
}

// pub fn get_config_path() -> String {
//     confy::get_configuration_file_path(APP_NAME, None)
//         .expect("configuration file path")
//         .as_path()
//         .to_str()
//         .expect("path string")
//         .to_string()
// }

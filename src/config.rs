/*!
 * The configuration file structure
 */

use serde::{Deserialize, Serialize};

/**
 * Configuration structure for ibflex
 */
#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub flex_query_id: String,
    pub ib_token: String,
}

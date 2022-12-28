/*!
 * Flex Statement request protocol
 */

use serde::{Deserialize, Serialize};

/**
 * Statement request protocol.
 */
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FlexStatementResponse {
    pub timestamp: String,

    pub Status: String,
    pub ReferenceCode: String,
    pub Url: String,
}

pub fn parse_response_text(text: &str) -> FlexStatementResponse {
    let statement: FlexStatementResponse = serde_xml_rs::from_str(text).expect("parsed statement");
    statement
}

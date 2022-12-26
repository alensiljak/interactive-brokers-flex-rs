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

pub fn parse_stmt_text(text: &String) -> FlexStatementResponse {
    let statement: FlexStatementResponse = serde_xml_rs::from_str(text).expect("parsed statement");
    statement
}

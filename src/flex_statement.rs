use serde::{Serialize, Deserialize};

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

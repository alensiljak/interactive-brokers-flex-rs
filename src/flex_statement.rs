/*!
 * Flex Statement request protocol
 *
 * https://www.interactivebrokers.com.au/en/software/etmug/employeetrack/flex%20web%20service%20version%203.htm
 */

use serde::{Deserialize, Serialize};

/**
The structured response from the Flex Query statement request.
Part of the Statement request protocol.

The tag names in the XML response are PascalCase. They are rewritten using serde rename attributes.
See tests.
 */
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FlexStatementResponse {
    #[serde(alias = "timestamp")] // This one is not PascalCase since it is an XML attribute.
    pub timestamp: String,

    pub status: String,
    pub reference_code: String,
    pub url: String,
}

pub fn parse_response_text(text: &str) -> FlexStatementResponse {
    let statement: FlexStatementResponse = serde_xml_rs::from_str(text).expect("parsed statement");
    statement
}

#[cfg(test)]
mod tests {
    use crate::flex_statement;

    /// Test parsing the XML response.
    /// The tags are PascalCase, attributes are camelCase, while the Rust struct members are
    /// snake_case.
    #[test]
    fn test_parsing_request_response() {
        let reqresp = r"<FlexStatementResponse timestamp='17 January, 2023 12:51 PM EST'>
<Status>Success</Status>
<ReferenceCode>1234567890</ReferenceCode>
<Url>https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.GetStatement</Url>
</FlexStatementResponse>
";
        let actual = flex_statement::parse_response_text(reqresp);

        println!("parsed: {:?}", actual);

        assert_eq!("Success", actual.status);
        assert_eq!("17 January, 2023 12:51 PM EST", actual.timestamp);
        assert_eq!("1234567890", actual.reference_code);
        assert_eq!("https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.GetStatement",
            actual.url);
    }
}

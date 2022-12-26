/*!
 * The module for download of IB Flex Query reports.
 *
 * <https://guides.interactivebrokers.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm>
 */

use chrono::Local;

const FLEX_URL: &str = "https://gdcdyn.interactivebrokers.com/Universal/servlet/";
const REQUEST_ENDPOINT: &str = "FlexStatementService.SendRequest";
const STMT_ENDPOINT: &str = "FlexStatementService.GetStatement";

/**
 * Downloads the Flex Query Cash Transactions report into a file in the current directory.
 */
pub async fn download() -> String {
    // download the report
    let cfg = crate::read_config();
    let report = download_report(&cfg.flex_query_id, &cfg.ib_token).await;

    // save to text file
    let today_date = Local::now().date_naive();
    let today = today_date.format("%Y-%m-%d");
    let output_filename = format!("{today}_cash-tx.xml");

    std::fs::write(&output_filename, report).expect("successfully saved");

    output_filename
}

/**
 * FlexQueryReport is downloaded in a 2-step process.
 * You need to supply the token (Reports / Settings / FlexWeb Service),
 * and the query id (Reports / Flex Queries / Custom Flex Queries / Configure).
 */
async fn download_report(query_id: &str, token: &str) -> String {
    let resp = request_statement(query_id, token).await;
    // parse
    let stmt_resp = crate::flex_statement::parse_response_text(&resp);

    // Now request the actual report.
    let stmt_text = download_statement_text(&stmt_resp.ReferenceCode, token).await;

    stmt_text
}

/**
 * Requests the statement. Receives the request id. 
 * Returns the text of the response, the content is xml.
 */
async fn request_statement(query_id: &str, token: &str) -> String {
    let url = format!("{FLEX_URL}{REQUEST_ENDPOINT}?v=3&t={token}&q={query_id}");

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("User-Agent", "Java")
        .send()
        .await
        .expect("response received");

    res.text().await.expect("contents of the response")
}

/**
 * Downloads the actual report. 2nd step.
 * Requires the reference code received in the 1st step.
 */
async fn download_statement_text(ref_code: &String, token: &str) -> String {
    let url = format!("{FLEX_URL}{STMT_ENDPOINT}?v=3&q={ref_code}&t={token}");

    reqwest::get(url)
        .await
        .expect("downloaded statement")
        .text()
        .await
        .expect("text response (xml)")
}

#[cfg(test)]
mod tests {
    use crate::{
        download::{request_statement, FLEX_URL, REQUEST_ENDPOINT},
        read_config,
    };

    use super::download_report;

    #[test]
    /// Test concatenating constants.
    fn constants_test() {
        let actual = format!("{FLEX_URL}{REQUEST_ENDPOINT}");

        assert_eq!("https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.SendRequest",
        actual);
    }

    /**
     * Test sending the Flex request. This is step 1 of the 2-step process.
     * 
     * To run the test, create ibflex.toml config and populate with valid parameters.
     */
    #[tokio::test]
    async fn request_report_test() {
        let cfg = read_config();
        let actual = request_statement(&cfg.flex_query_id, &cfg.ib_token).await;

        assert_ne!(String::default(), actual);
        assert!(!actual.contains("ERROR"));
    }

    /**
     * Request the full Flex report, using 2-step process.
     *
     * To run the test, create ibflex.toml config and populate with valid parameters.
     */
    #[tokio::test]
    async fn report_download_test() {
        let cfg = read_config();
        let result = download_report(&cfg.flex_query_id, &cfg.ib_token).await;

        assert!(result.contains("FlexQueryResponse"));
    }
}

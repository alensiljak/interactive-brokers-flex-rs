/*!
 * The module for download of IB Flex Query reports.
 *
 * <https://guides.interactivebrokers.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm>
 */

use chrono::Local;

use crate::{config::get_dl_config, flex_statement};

const FLEX_URL: &str = "https://gdcdyn.interactivebrokers.com/Universal/servlet/";
const REQUEST_ENDPOINT: &str = "FlexStatementService.SendRequest";
const STMT_ENDPOINT: &str = "FlexStatementService.GetStatement";

/**
 * Parameters for the download.
 */
#[derive(Debug, Default)]
pub struct DownloadParams {
    pub query_id: Option<u32>,
    pub token: Option<String>,
}

impl DownloadParams {
    pub fn new(query_id: Option<u32>, token: &Option<String>) -> Self {
        Self {
            query_id,
            token: match token {
                Some(tkn) => Some(tkn.to_owned()),
                None => None,
            },
        }
    }
}

/**
 * Downloads the Flex Query Cash Transactions report into a file in the current directory.
 */
pub async fn download(params: DownloadParams) -> String {
    // get the configuration
    let cfg = get_dl_config(params);

    // download the report
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
    let stmt_resp = flex_statement::parse_response_text(&resp);

    // Now request the actual report.
    let stmt_text = download_statement_text(&stmt_resp.reference_code, token).await;

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
    use crate::{download::{FLEX_URL, REQUEST_ENDPOINT, request_statement, DownloadParams}};

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
     * Uncomment the [tokio::test] line below.
     */
    // #[tokio::test]
    #[allow(unused)]
    async fn request_report_test() {
        let cfg = crate::config::get_dl_config(DownloadParams::default());
        let actual = request_statement(&cfg.flex_query_id, &cfg.ib_token).await;
        
        println!("received: {:?}", actual);

        assert_ne!(String::default(), actual);
        assert!(!actual.contains("ERROR"));

        assert!(false);
    }

    // /**
    //  * Request the full Flex report, using 2-step process.
    //  *
    //  * To run the test, create ibflex.toml config and populate with valid parameters.
    //  */
    // #[tokio::test]
    // async fn report_download_test() {
    //     let cfg = crate::config::get_dl_config(DownloadParams::default());
    //     let result = download_report(&cfg.flex_query_id, &cfg.ib_token).await;
    //     assert!(result.contains("FlexQueryResponse"));
    // }
}

/*!
 * The module for download of IB Flex Query reports.
 *
 * <https://guides.interactivebrokers.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm>
 */

use chrono::Local;

use crate::flex_statement;

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
pub fn download(params: DownloadParams) -> String {
    let query_id = match params.query_id {
        Some(qid) => qid.to_string(),
        None => panic!("The query id is mandatory for the report download!"),
    };
    let token = match params.token {
        Some(tkn) => tkn,
        None => panic!("The token is mandatory for the report download!"),
    };

    let report = download_report(&query_id, &token);

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
fn download_report(query_id: &str, token: &str) -> String {
    let resp = request_statement(query_id, token);
    let stmt_resp = flex_statement::parse_response_text(&resp)
        .unwrap_or_else(|e| panic!("Parse failed: {e}\nRaw response:\n{resp}"));

    // Wait before requesting the actual report, as IB needs time to prepare it.
    for i in (1..=5).rev() {
        print!("\rDownloading report in {i}s...");
        std::io::Write::flush(&mut std::io::stdout()).ok();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!();

    download_statement_text(&stmt_resp.reference_code, token)
}

/**
 * Requests the statement. Receives the request id.
 * Returns the text of the response, the content is xml.
 */
fn request_statement(query_id: &str, token: &str) -> String {
    let url = format!("{FLEX_URL}{REQUEST_ENDPOINT}?v=3&t={token}&q={query_id}");
    ureq::get(&url)
        .set("User-Agent", "Java")
        .call()
        .expect("response received")
        .into_string()
        .expect("contents of the response")
}

/**
 * Downloads the actual report. 2nd step.
 * Requires the reference code received in the 1st step.
 */
fn download_statement_text(ref_code: &str, token: &str) -> String {
    let url = format!("{FLEX_URL}{STMT_ENDPOINT}?v=3&q={ref_code}&t={token}");
    ureq::get(&url)
        .call()
        .expect("downloaded statement")
        .into_string()
        .expect("text response (xml)")
}

#[cfg(test)]
mod tests {
    use crate::download::{FLEX_URL, REQUEST_ENDPOINT};

    #[test]
    /// Test concatenating constants.
    fn constants_test() {
        let actual = format!("{FLEX_URL}{REQUEST_ENDPOINT}");

        assert_eq!(
            "https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.SendRequest",
            actual
        );
    }

    /**
     * Test sending the Flex request. This is step 1 of the 2-step process.
     *
     * To run the test, create ibflex.toml config and populate with valid parameters.
     * Uncomment the [test] line below.
     */
    // #[test]
    #[allow(unused)]
    fn request_report_test() {
        use super::{request_statement, DownloadParams};
        let cfg = DownloadParams::default();
        let actual = request_statement(
            &cfg.query_id.unwrap().to_string(),
            &cfg.token.unwrap(),
        );

        println!("received: {:?}", actual);

        assert_ne!(String::default(), actual);
        assert!(!actual.contains("ERROR"));

        assert!(false);
    }
}

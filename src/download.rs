use chrono::Local;
use const_format::concatcp;

const FLEX_URL: &str = "https://gdcdyn.interactivebrokers.com/Universal/servlet/";
const REQUEST_URL: &str = concatcp!(FLEX_URL, "FlexStatementService.SendRequest");
const STMT_URL: &str = concatcp!(FLEX_URL, "FlexStatementService.GetStatement");

/**
 * The module for download of IB Flex reports.
 *
 * https://guides.interactivebrokers.com/reportingreference/reportguide/activity%20flex%20query%20reference.htm
 *
 * Old:
 * https://www.interactivebrokers.com/en/software/am/am/reports/flex_web_service_version_3.htm
 */
pub async fn download() {
    let today_date = Local::now().date_naive();
    let today = today_date.format("%Y-%m-%d");
    let output_filename = format!("{today}_cash-tx.xml");

    let cfg = crate::read_config();

    let report = download_report(&cfg.flex_query_id, &cfg.ib_token).await;

    // save to the output
    std::fs::write(output_filename, report).expect("successfully saved");
}

/**
 * FlexQueryReport is downloaded in a 2-step process.
 * You need to supply your token (Reports / Settings / FlexWeb Service),
 * and the query id (Reports / Flex Queries / Custom Flex Queries / Configure).
 */
pub async fn download_report(query_id: &str, token: &str) -> String {
    request_statement(query_id, token).await
}

async fn request_statement(query_id: &str, token: &str) -> String {
    let url = format!("{REQUEST_URL}{}{}{token}{}{query_id}", "?v=3", ",t=", ",q=");

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("User-Agent", "Java")
        .send()
        .await
        .expect("response received");

    res.text().await.expect("contents of the response")

    // log::debug!("result of the request is {:?}", x);
}

#[cfg(test)]
mod tests {
    use crate::{
        download::{request_statement, REQUEST_URL},
        read_config,
    };

    use super::download_report;

    #[test]
    /// Test concatenating constants.
    fn constants_test() {
        assert_eq!("https://gdcdyn.interactivebrokers.com/Universal/servlet/FlexStatementService.SendRequest",
        REQUEST_URL);
    }

    #[tokio::test]
    async fn report_download_test() {
        let cfg = read_config();
        let result = download_report(&cfg.flex_query_id, &cfg.ib_token).await;

        assert_eq!("yo".to_string(), result);
    }

    /**
     * To run the test, create ibflex.toml config and populate with valid parameters.
     */
    #[tokio::test]
    async fn request_report_test() {
        let cfg = read_config();
        let actual = request_statement(&cfg.flex_query_id, &cfg.ib_token).await;

        assert_eq!(String::default(), actual);
    }
}

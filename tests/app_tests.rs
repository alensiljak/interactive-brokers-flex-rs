/*!
 * Test the app
 */

use std::env;

use ibflex::{compare::CompareParams, download::DownloadParams};

/**
 * Define a database to use.
 */
#[rstest::fixture]
fn database() {}

#[rstest::fixture]
fn app() {}

#[tokio::test]
async fn test_download_w_params() {
    let params = DownloadParams::new(Some(12345_u32), &Some("12345".to_string()));
    let actual = ibflex::download::download(params).await;

    // println!("got {:?}", actual);

    assert_ne!(String::default(), actual);
    assert!(actual.contains("xml"));
}

#[rstest::rstest]
fn test_comparison() {
    // prepare
    env::set_var("token", "123");

    let params = CompareParams {
        flex_report_path: None,
        ledger_init_file: None,
    };
    let actual = ibflex::compare::compare(params);

    assert!(!actual.is_err());
}

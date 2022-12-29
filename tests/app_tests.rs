/*!
 * Test the app
 */

// use std::env;

// use ibflex::{compare::CompareParams, download::DownloadParams};

// #[tokio::test]
// async fn test_download_w_params() {
//     let params = DownloadParams::new(Some(12345_u32), &Some("12345".to_string()));
//     let actual = ibflex::download::download(params).await;

//     // println!("got {:?}", actual);

//     assert_ne!(String::default(), actual);
//     assert!(actual.contains("xml"));
// }
    //env::set_var("token", "123");

use assert_cmd::Command;

/**
 * Tests comparison.
 * Requires a default .toml configuration file.
 */
#[rstest::rstest]
fn test_comparison() {
    // prepare

    let mut cmd = Command::cargo_bin("ibflex").unwrap();

    let assert = cmd.args(vec!["cmp"]).assert();
    //assert!(!actual.is_err());
    assert.success();
    
    todo!("add assertions");
}

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

    let assert = cmd.arg("cmp").assert();
    // .args(vec!["cmp"])
    //assert!(!actual.is_err());

    // Assertions
    let expected = r#"New: 2022-11-25/2022-11-25 DGS     Commission Adjustments 0.33225725 USD, Refund (DGS, 10, 2022-10-26)
New: 2022-11-30/2022-11-30         Deposits/Withdrawals    1500 EUR, CASH RECEIPTS / ELECTRONIC FUND TRANSFERS
New: 2022-12-05/2022-12-05         Broker Interest Received    2.77 AUD, AUD CREDIT INT FOR NOV-2022
New: 2022-12-14/2022-12-15 TCBT_AS Div    6.05 EUR, TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE (Ordinary Dividend)
New: 2022-12-14/2022-12-15 TCBT_AS Tax   -0.91 EUR, TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX
Complete.
"#;

    /* These should be recognized from the ledger:
    New: 2022-12-15/2022-12-15 TRET_AS Div    38.4 EUR, TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE (Ordinary Dividend)
    New: 2022-12-15/2022-12-15 TRET_AS Tax   -5.77 EUR, TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE - NL TAX
     */
    
    assert.success().stdout(expected);
}

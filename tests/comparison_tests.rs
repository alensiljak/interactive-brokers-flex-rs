/*!
 * Test the Compare method.
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
use rstest::fixture;

#[fixture]
fn app_cmd() -> Command {
    Command::cargo_bin("ibflex").unwrap()
}

/**
 * Tests comparison.
 * Requires a default .toml configuration file.
 */
#[rstest::rstest]
fn test_comparison_actual_date(mut app_cmd: Command) {
    // prepare
    let assert = app_cmd
        .args([
            "cmp",
            "--flex-report-path",
            "tests/report_1.xml",
            "--symbols-path",
            "tests/symbols.csv",
            "--ledger-init-file",
            "tests/init.ledger",
        ])
        .assert();

    // Assertions
    let expected = r#"Using tests/report_1.xml
Skipped: 2022-11-25/2022-11-25 DGS     ARCA Commission Adjustments 0.33225725 USD, Refund (DGS, 10, 2022-10-26)
Skipped: 2022-11-30/2022-11-30          Deposits/Withdrawals    1500 EUR, CASH RECEIPTS / ELECTRONIC FUND TRANSFERS
Skipped: 2022-12-05/2022-12-05          Broker Interest Received    2.77 AUD, AUD CREDIT INT FOR NOV-2022
New: 2022-12-14/2022-12-15 TCBT_AS Dividend    6.05 EUR, TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE (Ordinary Dividend)
New: 2022-12-14/2022-12-15 TCBT_AS WhTax      -0.91 EUR, TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX
Complete.
"#;

    /* These should be recognized from the ledger:
    New: 2022-12-15/2022-12-15 TRET_AS Dividend    38.4 EUR, TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE (Ordinary Dividend)
    New: 2022-12-15/2022-12-15 TRET_AS WhTax   -5.77 EUR, TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE - NL TAX
     */

    assert.success().stdout(expected);
}

#[rstest::rstest]
fn test_comparison_w_effective_dates(mut app_cmd: Command) {
    let assert = app_cmd
        .args(vec![
            "cmp",
            "--effective",
            "--flex-report-path",
            "tests/tax_adj_report.xml",
            "--symbols-path",
            "tests/symbols.csv",
            "--ledger-init-file",
            "tests/tax_adj.ledgerrc",
        ])
        .assert();

    let expected = r#"Using tests/tax_adj_report.xml
New: 2023-01-24/2022-04-01 BBN     WhTax       0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-01 BBN     WhTax      -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax       0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax      -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
Complete.
"#;

/* These should be recognized:
New: 2023-01-24/2022-12-01 BBN     WhTax    0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-12-01 BBN     WhTax   -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
 */

    assert.success().stdout(expected);
}

#[rstest::rstest]
fn test_start_date_parameter(mut app_cmd: Command) {
    let assert = app_cmd
        .args(vec![
            "cmp",
            "--flex-report-path",
            "tests/report_1.xml",
            "--symbols-path",
            "tests/symbols.csv",
            "--ledger-init-file",
            "tests/init.ledger",
        ])
        .assert();

    assert.success();
}

#[rstest::rstest]
fn test_tcf(mut app_cmd: Command) {
    let assert = app_cmd
        .args(vec![
            "cmp",
            "--flex-report-path",
            "tests/tcf.xml",
            "--symbols-path",
            "tests/symbols.csv",
            "--ledger-journal-file",
            "tests/tcf.ledger",
        ])
        .assert();

    assert.success();
}
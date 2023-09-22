//! Testing the functionality

use assert_cmd::Command;

#[rstest::rstest]
fn test_same_symbols_different_exchange() {
    let mut app_cmd = Command::cargo_bin("ibflex").unwrap();
    let cmd = "cmp --flex-report-path tests/same_symbol.xml --symbols-path tests/symbols.csv --ledger-journal-file tests/journal.ledger";
    let args = shell_words::split(cmd).unwrap();
    let assert = app_cmd.args(args).assert();

    let expected = r#"Using tests/same_symbol.xml
New: 2023-09-14/2023-09-15 SDIV    Dividend    5.04 USD, SDIV(US37960A6698) CASH DIVIDEND USD 0.21 PER SHARE (Ordinary Dividend)
New: 2023-09-21/2023-09-22 SDIV    Dividend   10.26 USD, SDIV(IE00077FRP95) CASH DIVIDEND USD 0.09 PER SHARE (Mixed Income)
Complete.
"#;

    assert.success().stdout(expected);

    // assert!(!actual.is_empty());
    // assert_eq!(expected, actual);
}

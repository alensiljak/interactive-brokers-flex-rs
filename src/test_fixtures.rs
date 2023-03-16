/*!
 * Test fixtures
 */

use std::path::PathBuf;

use chrono::Local;
use rstest::fixture;

use crate::{compare::CompareParams, flex_query::CashTransaction, ISO_DATE_FORMAT};

#[fixture]
pub fn tests_directory_path() -> String {
    "tests/".to_string()
}
#[fixture]
pub fn flex_report_path(tests_directory_path: String) -> String {
    let path = format!("{tests_directory_path}{}", "report_1.xml");
    // canonicalize(path)

    path
}

#[fixture]
pub fn symbols_path(tests_directory_path: String) -> PathBuf {
    let path = tests_directory_path + "/symbols.csv"; 
    PathBuf::from(path)
}

#[fixture]
pub fn ledger_journal_path(tests_directory_path: String) -> String {
    let path = format!("{tests_directory_path}{}", "journal.ledger");

    path
}

#[fixture]
pub fn cmp_params(flex_report_path: String, ledger_journal_path: String,
    symbols_path: PathBuf) -> CompareParams {
    CompareParams {
        flex_report_path: Some(flex_report_path),
        flex_reports_dir: None,
        ledger_journal_file: Some(ledger_journal_path),
        symbols_path: symbols_path.as_path().to_str().unwrap().to_owned(),
        effective_dates: false,
    }
}

#[fixture]
pub fn cash_transactions() -> Vec<CashTransaction> {
    let tx1 = CashTransaction {
        report_date: "2022-12-14".to_string(),
        date_time: "2022-12-15;12:20:00".to_string(),
        amount: "-0.91".to_string(),
        currency: "EUR".to_string(),
        description: "TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX".to_string(),
        listing_exchange: "AEB".to_string(),
        symbol: "TCBT".to_string(),
        r#type: "Withholding Tax".to_string(),
    };

    let dist = CashTransaction {
        report_date: Local::now().format(ISO_DATE_FORMAT).to_string(),
        date_time: "2022-12-26".into(),
        amount: "10".into(),
        currency: "EUR".into(),
        description: "TCBT distribution".into(),
        r#type: "Dividends".into(),
        listing_exchange: "AMS".into(),
        symbol: "TCBT".into(),
    };

    vec![tx1, dist]
}

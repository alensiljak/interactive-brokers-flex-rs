/*!
 * Test fixtures
 */

use chrono::Local;
use rstest::fixture;

use crate::{compare::CompareParams, flex_query_def::CashTransaction, ISO_DATE_FORMAT, config::{Config, get_cmp_config}};

#[fixture]
pub fn tests_directory_path() -> String {
    let cur_dir = std::env::current_dir().expect("current directory");
    let tests_path = format!(
        "{}{}{}{}",
        cur_dir.display(),
        std::path::MAIN_SEPARATOR,
        "tests",
        std::path::MAIN_SEPARATOR,
    );

    // canonicalize("./tests/").expect("valid path")
    //     .as_os_str().to_str().expect("string ref")
    //     .to_owned()

    tests_path
}
#[fixture]
pub fn flex_report_path(tests_directory_path: String) -> String {
    let path = format!("{tests_directory_path}{}", "report_1.xml");
    // canonicalize(path)

    path
}

#[fixture]
pub fn ledger_init_path(tests_directory_path: String) -> String {
    let path = format!("{tests_directory_path}{}", "init.ledger");

    path
}

#[fixture]
pub fn cmp_params(flex_report_path: String, ledger_init_path: String) -> CompareParams {
    CompareParams {
        flex_report_path: Some(flex_report_path),
        flex_reports_dir: None,
        ledger_init_file: Some(ledger_init_path),
    }
}

#[fixture]
pub fn cmp_config(cmp_params: CompareParams) -> Config {
    get_cmp_config(&cmp_params)
}

#[fixture]
pub fn cash_transactions() -> Vec<CashTransaction> {
    let tx1 = CashTransaction {
        reportDate: "2022-12-14".to_string(),
        dateTime: "2022-12-15;12:20:00".to_string(),
        amount: "-0.91".to_string(),
        currency: "EUR".to_string(),
        description: "TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX".to_string(),
        listingExchange: "AEB".to_string(),
        symbol: "TCBT".to_string(),
        r#type: "Withholding Tax".to_string(),
    };

    let dist = CashTransaction {
        reportDate: Local::now().format(ISO_DATE_FORMAT).to_string(),
        dateTime: "2022-12-26".into(),
        amount: "10".into(),
        currency: "EUR".into(),
        description: "TCBT distribution".into(),
        r#type: "DIST".into(),
        listingExchange: "AMS".into(),
        symbol: "TCBT".into(),
    };

    vec![tx1, dist]
}

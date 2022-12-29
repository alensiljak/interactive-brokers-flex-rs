/*!
 * Test fixtures
 */

use rstest::fixture;

use crate::{flex_query_def::CashTransaction, compare::CompareParams};

#[fixture]
pub fn flex_report_path() -> String {
    let cur_dir = std::env::current_dir().expect("current directory");
    let path = format!(
        "{}{}{}{}{}",
        cur_dir.display(),
        std::path::MAIN_SEPARATOR,
        "tests",
        std::path::MAIN_SEPARATOR,
        "report_1.xml"
    );
    // canonicalize(path)

    path
}

#[fixture]
pub fn cmp_params(flex_report_path: String) -> CompareParams {
    let ledger_init_file = None;
    CompareParams {
        flex_report_path: Some(flex_report_path),
        ledger_init_file,
    }
}

#[fixture]
pub fn cash_transactions() -> Vec<CashTransaction> {
    let tx1 = CashTransaction {
        reportDate: "2022-12-14".to_string(),
        amount: "-0.91".to_string(),
        currency: "EUR".to_string(),
        dateTime: "2022-12-15;12:20:00".to_string(),
        description: "TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX".to_string(),
        listingExchange: "AEB".to_string(),
        symbol: "TCBT".to_string(),
        r#type: "Withholding Tax".to_string(),
    };

    let dist = CashTransaction {
        reportDate: "today".into(),
        amount: "10".into(),
        currency: "EUR".into(),
        dateTime: "2022-12-26".into(),
        description: "TCBT distribution".into(),
        r#type: "DIST".into(),
        listingExchange: "AMS".into(),
        symbol: "TCBT".into(),
    };

    vec![tx1, dist]
}
/*!
 * Test fixtures
 */

use rstest::fixture;

use crate::flex_query_def::CashTransaction;

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
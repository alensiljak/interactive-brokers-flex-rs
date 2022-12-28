/*!
 * The domain model.
 */

use std::str::FromStr;

use rust_decimal::Decimal;

use crate::flex_query_def::CashTransaction;

/**
 * The ledger transaction record.
 * Used for comparison between IB (translated) and Ledger records.
 */
#[derive(Default)]
pub struct LedgerTransaction {
    date: String,
    report_date: String,
    // effective_date: str = None
    payee: String,
    account: String,
    amount: Decimal,
    currency: String,
    symbol: String, // required for IB Cash Transactions
    r#type: String,
}

const ISO_DATE_FMT: &str = "%Y-%m-%d";

impl From<&CashTransaction> for LedgerTransaction {
    fn from(value: &CashTransaction) -> Self {
        // prepare symbol
        let mut symbol = value.symbol.to_owned();
        // Eliminate 'd' at the end of the symbol.
        if symbol.ends_with('d') {
            symbol = symbol[..symbol.len() - 1].to_string();
        }

        LedgerTransaction {
            date: value.dateTime.to_owned(),
            report_date: value.reportDate.to_owned(),
            payee: String::default(),   // not used
            account: String::default(), // not used
            amount: Decimal::from_str(value.amount.as_str()).unwrap(),
            currency: String::default(), // not used
            symbol,
            r#type: match value.r#type.as_str() {
                "DIVIDEND" => "Div".to_string(),
                "WHTAX" => "Tax".to_string(),
                _ => value.r#type.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    use crate::{flex_query_def::CashTransaction, model::LedgerTransaction};

    #[fixture]
    fn cash_transactions() -> Vec<CashTransaction> {
        crate::test_fixtures::cash_transactions()
    }

    #[rstest]
    fn conversion_test(cash_transactions: Vec<CashTransaction>) {
        let t1: LedgerTransaction = LedgerTransaction::from(&cash_transactions[0]);

        // assert
        assert_eq!(String::default(), t1.account);
        assert_eq!(Decimal::from_str("-0.91").unwrap(), t1.amount);
    }
}

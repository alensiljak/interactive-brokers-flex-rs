/*!
 * The domain model.
 */

use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDateTime, NaiveDate};
use rust_decimal::Decimal;

use crate::{flex_query::CashTransaction, ISO_DATE_FORMAT, flex_enums};

/**
 * The ledger transaction record.
 * Used for comparison between IB (translated) and Ledger records.
 */
#[derive(Debug, Default, PartialEq, Eq)]
pub struct CommonTransaction {
    pub date: NaiveDateTime,
    pub report_date: String,
    // effective_date: str = None
    pub payee: String,
    pub account: String,
    pub amount: Decimal,
    pub currency: String,
    pub symbol: String, // required for IB Cash Transactions
    pub r#type: String,
    pub description: String,
}

// const ISO_DATE_FMT: &str = "%Y-%m-%d";

impl From<&CashTransaction> for CommonTransaction {
    fn from(value: &CashTransaction) -> Self {
        log::debug!("converting ib tx: {:?}", value);

        // prepare symbol
        let mut symbol = value.symbol.to_owned();
        // Eliminate 'd' at the end of the symbol.
        if symbol.ends_with('d') {
            symbol = symbol[..symbol.len() - 1].to_string();
        }

        CommonTransaction {
            date: match value.date_time.len() {
                10 => {
                    log::debug!("the date is {}", value.date_time);

                    let tx_date = NaiveDate::parse_from_str(&value.date_time.as_str(), ISO_DATE_FORMAT)
                    .expect("valid date expected");
                    NaiveDateTime::from(tx_date.and_hms_opt(0, 0, 0).unwrap())
                },
                19 => NaiveDateTime::parse_from_str(&value.date_time.as_str(), "%Y-%m-%d;%H:%M:%S")
                    .expect("valid date/time expected"),
                _ => panic!("Invalid date/time"),
            },
            report_date: value.report_date.to_owned(),
            payee: String::default(),   // not used
            account: String::default(), // not used
            amount: Decimal::from_str(value.amount.as_str()).unwrap(),
            currency: value.currency.to_owned(),
            symbol,
            r#type: flex_enums::cash_action(value.r#type.as_str()).to_owned(),
            description: value.description.to_owned(),
        }
    }
}

impl Display for CommonTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} {:7} {} {:>7} {}, {}",
            self.report_date,
            self.date.date(),
            self.symbol,
            self.r#type,
            self.amount,
            self.currency,
            self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    use crate::{flex_query::CashTransaction, model::CommonTransaction};

    #[fixture]
    fn cash_transactions() -> Vec<CashTransaction> {
        crate::test_fixtures::cash_transactions()
    }

    #[rstest]
    fn conversion_test(cash_transactions: Vec<CashTransaction>) {
        let t1 = CommonTransaction::from(&cash_transactions[0]);

        // assert
        assert_eq!(String::default(), t1.account);
        assert_eq!(Decimal::from_str("-0.91").unwrap(), t1.amount);
    }
}

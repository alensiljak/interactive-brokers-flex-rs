/*!
 * Definitions for Flex Query report
 */

use std::fmt::Display;

use serde::Deserialize;

/**
 * The structure of the IB Flex report.
 */
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexQueryResponse {
    #[serde(rename = "FlexStatements")]
    pub flex_statements: FlexStatements,
}

impl From<String> for FlexQueryResponse {
    /**
     * Parses the file contents (xml) into the FlexQueryResponse object.
     */
    fn from(value: String) -> Self {
        serde_xml_rs::from_str(&value).expect("parsed XML")
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexStatements {
    pub count: i32,

    #[serde(rename = "FlexStatement")]
    pub flex_statement: FlexStatement,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexStatement {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "fromDate")]
    pub from_date: String,
    #[serde(rename = "toDate")]
    pub to_date: String,
    pub period: String,
    #[serde(rename = "whenGenerated")]
    pub when_generated: String,

    #[serde(rename = "CashTransactions")]
    pub cash_transactions: CashTransactions,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction")]
    pub cash_transaction: Vec<CashTransaction>,
}

/**
 * .report_date is the real date, when the transaction appears in the IB report.
 * .date is the transaction effective date.
 */
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CashTransaction {
    /// .report_date is the real date, when the transaction appears in the IB report.
    #[serde(rename = "reportDate")]
    pub report_date: String,
    /// .dateTime is the transaction effective date.
    #[serde(rename = "dateTime")]
    pub date_time: String,
    pub symbol: String,
    #[serde(rename = "listingExchange")]
    pub listing_exchange: String,
    pub r#type: String,
    pub amount: String,
    pub currency: String,
    pub description: String,
}

impl Display for CashTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} {:7} {} {} {:>7} {}, {}",
            self.report_date,
            &self.date_time[..10],
            self.symbol,
            self.listing_exchange,
            self.r#type,
            self.amount,
            self.currency,
            self.description
        )

    }
}

// pub enum TxType {
//     "Deposits/Withdrawals",
//     Dividends,
//     WithholdingTax
// }

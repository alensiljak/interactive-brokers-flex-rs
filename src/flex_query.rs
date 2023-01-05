/*!
 * Definitions for Flex Query report
 */

use serde::Deserialize;

/**
 * The structure of the IB Flex report.
 */
#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexQueryResponse {
    pub FlexStatements: FlexStatements,
}

impl From<String> for FlexQueryResponse {
    /**
     * Parses the file contents (xml) into the FlexQueryResponse object.
     */
    fn from(value: String) -> Self {
        serde_xml_rs::from_str(&value).expect("parsed XML")
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexStatements {
    pub count: i32,
    pub FlexStatement: FlexStatement,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexStatement {
    pub accountId: String,
    pub fromDate: String,
    pub toDate: String,
    pub period: String,
    pub whenGenerated: String,

    pub CashTransactions: CashTransactions,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CashTransactions {
    pub CashTransaction: Vec<CashTransaction>,
}

/**
 * .report_date is the real date, when the transaction appears in the IB report.
 * .date is the transaction effective date.
 */
#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CashTransaction {
    /// .report_date is the real date, when the transaction appears in the IB report.
    pub reportDate: String,
    /// .dateTime is the transaction effective date.
    pub dateTime: String,
    pub symbol: String,
    pub listingExchange: String,
    pub r#type: String,
    pub amount: String,
    pub currency: String,
    pub description: String,
}

// pub enum TxType {
//     "Deposits/Withdrawals",
//     Dividends,
//     WithholdingTax
// }

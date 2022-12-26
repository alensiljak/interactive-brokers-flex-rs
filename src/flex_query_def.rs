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

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct FlexStatements {
    pub count: i32,
    pub FlexStatement: FlexStatement
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
    pub CashTransaction: Vec<CashTransaction>
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct CashTransaction {
    pub reportDate: String,
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



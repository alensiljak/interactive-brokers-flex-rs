/*!
 * Definitions for Flex Query report
 */

use std::fmt::Display;

use serde::Deserialize;

/**
 * The structure of the IB Flex report.
 */
#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct FlexQueryResponse {
    #[serde(rename = "FlexStatements", default)]
    pub flex_statements: FlexStatements,
}

impl FlexQueryResponse {
    /// Parses the file contents (xml) into the [`FlexQueryResponse`] object.
    pub fn from_xml(xml: &str) -> anyhow::Result<Self> {
        Ok(quick_xml::de::from_str(xml)?)
    }
}

impl From<String> for FlexQueryResponse {
    /**
     * Parses the file contents (xml) into the FlexQueryResponse object.
     */
    fn from(value: String) -> Self {
        quick_xml::de::from_str(&value).expect("parsed XML")
    }
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct FlexStatements {
    #[serde(rename = "@count", default)]
    pub count: i32,

    #[serde(rename = "FlexStatement", default)]
    pub flex_statement: Vec<FlexStatement>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct FlexStatement {
    #[serde(rename = "@accountId", default)]
    pub account_id: String,
    #[serde(rename = "@fromDate", default)]
    pub from_date: String,
    #[serde(rename = "@toDate", default)]
    pub to_date: String,
    #[serde(rename = "@period", default)]
    pub period: String,
    #[serde(rename = "@whenGenerated", default)]
    pub when_generated: String,

    #[serde(rename = "Trades", default)]
    pub trades: Trades,
    #[serde(rename = "CashTransactions", default)]
    pub cash_transactions: CashTransactions,
    #[serde(rename = "CashReport", default)]
    pub cash_report: CashReport,
    #[serde(rename = "OpenPositions", default)]
    pub open_positions: OpenPositions,
    #[serde(rename = "CorporateActions", default)]
    pub corporate_actions: CorporateActions,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CashTransactions {
    #[serde(rename = "CashTransaction", default)]
    pub cash_transaction: Vec<CashTransaction>,
}

/**
 * .report_date is the real date, when the transaction appears in the IB report.
 * .date is the transaction effective date.
 */
#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CashTransaction {
    /// .report_date is the real date, when the transaction appears in the IB report.
    #[serde(rename = "@reportDate", default)]
    pub report_date: String,
    /// .dateTime is the transaction effective date.
    #[serde(rename = "@dateTime", default)]
    pub date_time: String,
    #[serde(rename = "@symbol", default)]
    pub symbol: String,
    #[serde(rename = "@isin", default)]
    pub isin: String,
    #[serde(rename = "@listingExchange", default)]
    pub listing_exchange: Option<String>,
    #[serde(rename = "@type", default)]
    pub r#type: String,
    #[serde(rename = "@amount", default)]
    pub amount: String,
    #[serde(rename = "@currency", default)]
    pub currency: String,
    #[serde(rename = "@description", default)]
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
            self.listing_exchange.as_deref().unwrap_or(""),
            self.r#type,
            self.amount,
            self.currency,
            self.description
        )

    }
}

/// Trades section. The XML preserves the interleaved order of `<Trade>` and
/// `<Lot>` elements; consumers that need lot-matching should iterate `items`.
#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct Trades {
    #[serde(rename = "$value", default)]
    pub items: Vec<TradeItem>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum TradeItem {
    Trade(Trade),
    Lot(Lot),
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct Trade {
    #[serde(rename = "@reportDate", default)]
    pub report_date: String,
    #[serde(rename = "@currency", default)]
    pub currency: String,
    #[serde(rename = "@symbol", default)]
    pub symbol: String,
    #[serde(rename = "@isin", default)]
    pub isin: String,
    #[serde(rename = "@dateTime", default)]
    pub date_time: String,
    #[serde(rename = "@transactionType", default)]
    pub transaction_type: String,
    #[serde(rename = "@quantity", default)]
    pub quantity: String,
    #[serde(rename = "@tradePrice", default)]
    pub trade_price: String,
    #[serde(rename = "@tradeMoney", default)]
    pub trade_money: String,
    #[serde(rename = "@proceeds", default)]
    pub proceeds: String,
    #[serde(rename = "@ibCommission", default)]
    pub ib_commission: String,
    #[serde(rename = "@ibCommissionCurrency", default)]
    pub ib_commission_currency: String,
    #[serde(rename = "@netCash", default)]
    pub net_cash: String,
    #[serde(rename = "@cost", default)]
    pub cost: String,
    #[serde(rename = "@taxes", default)]
    pub taxes: String,
    #[serde(rename = "@buySell", default)]
    pub buy_sell: String,
    #[serde(rename = "@openCloseIndicator", default)]
    pub open_close_indicator: String,
    #[serde(rename = "@tradeDate", default)]
    pub trade_date: String,
    #[serde(rename = "@openDateTime", default)]
    pub open_date_time: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct Lot {
    #[serde(rename = "@currency", default)]
    pub currency: String,
    #[serde(rename = "@symbol", default)]
    pub symbol: String,
    #[serde(rename = "@isin", default)]
    pub isin: String,
    #[serde(rename = "@dateTime", default)]
    pub date_time: String,
    #[serde(rename = "@quantity", default)]
    pub quantity: String,
    #[serde(rename = "@cost", default)]
    pub cost: String,
    #[serde(rename = "@openDateTime", default)]
    pub open_date_time: String,
    #[serde(rename = "@reportDate", default)]
    pub report_date: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CashReport {
    #[serde(rename = "CashReportCurrency", default)]
    pub cash_report_currency: Vec<CashReportCurrency>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CashReportCurrency {
    #[serde(rename = "@currency", default)]
    pub currency: String,
    #[serde(rename = "@toDate", default)]
    pub to_date: String,
    #[serde(rename = "@endingCash", default)]
    pub ending_cash: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct OpenPositions {
    #[serde(rename = "OpenPosition", default)]
    pub open_position: Vec<OpenPosition>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct OpenPosition {
    #[serde(rename = "@symbol", default)]
    pub symbol: String,
    #[serde(rename = "@isin", default)]
    pub isin: String,
    #[serde(rename = "@position", default)]
    pub position: String,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CorporateActions {
    #[serde(rename = "CorporateAction", default)]
    pub corporate_action: Vec<CorporateAction>,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct CorporateAction {
    #[serde(rename = "@actionID", default)]
    pub action_id: String,
    #[serde(rename = "@reportDate", default)]
    pub report_date: String,
    #[serde(rename = "@dateTime", default)]
    pub date_time: String,
    #[serde(rename = "@currency", default)]
    pub currency: String,
    #[serde(rename = "@symbol", default)]
    pub symbol: String,
    #[serde(rename = "@isin", default)]
    pub isin: String,
    #[serde(rename = "@type", default)]
    pub r#type: String,
    #[serde(rename = "@description", default)]
    pub description: String,
    #[serde(rename = "@amount", default)]
    pub amount: String,
    #[serde(rename = "@proceeds", default)]
    pub proceeds: String,
    #[serde(rename = "@value", default)]
    pub value: String,
    #[serde(rename = "@quantity", default)]
    pub quantity: String,
}

// pub enum TxType {
//     "Deposits/Withdrawals",
//     Dividends,
//     WithholdingTax
// }

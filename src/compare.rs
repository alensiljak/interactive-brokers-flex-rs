/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::collections::HashMap;

use anyhow::{Error, Ok};

use crate::{
    config::{get_cmp_config, Config},
    flex_query_def::{CashTransaction, FlexQueryResponse},
    flex_query_reader::load_report,
    model::CommonTransaction, ledger_runner,
};

pub const TRANSACTION_DAYS: u8 = 60;
pub(crate) const DATE_MODE: &str = "book"; // "book" / "effective"

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(params: CompareParams) -> anyhow::Result<()> {
    log::debug!("comparing distributions, params: {:?}", params);
    let cfg = get_cmp_config(&params);

    // get_ib_report_tx
    let ib_tx = get_ib_tx(&cfg);

    // get_ledger_tx
    let ledger_tx = ledger_runner::get_ledger_tx(cfg.ledger_init_file);

    // compare
    compare_txs(ib_tx, ledger_tx);

    Ok(())
}

/// Load symbol mappings from PriceDb
fn load_symbols() -> Result<HashMap<String, String>, Error> {
    log::debug!("loading symbols");

    // read symbols from pricedb
    let cfg = pricedb::load_config();
    let pdb = pricedb::App::new(cfg);
    // get all the securities that have a different symbol in ledger.
    let securities: HashMap<String, String> = pdb
        .get_dal()
        .get_securities(None)
        .into_iter()
        .filter_map(|sec| match sec.ledger_symbol {
            Some(ledger_symbol) => Some((sec.symbol, ledger_symbol)),
            None => None,
        })
        .collect();

    Ok(securities)
}

/**
 * Returns transactions from the Flex Report, for comparison.
 * symbols is a HashMap of symbol rewrites.
 */
fn get_ib_tx(cfg: &Config) -> Vec<CommonTransaction> {
    let ib_txs = read_flex_report(cfg);

    convert_ib_txs(ib_txs)
}

fn convert_ib_txs(ib_txs: Vec<CashTransaction>) -> Vec<CommonTransaction> {
    // load symbols
    let symbols = load_symbols().unwrap();
    let mut txs: Vec<CommonTransaction> = vec![];

    let skip = ["WHTAX", "DIVIDEND"];
    for tx in ib_txs {
        // skip any not matching the expected types.
        if skip.iter().any(|t| *t == tx.r#type) {
            println!("Skipping...");
        }

        let mut ltx: CommonTransaction = (&tx).into();

        // use adjusted symbols
        if symbols.contains_key(&ltx.symbol) {
            // log::debug!("adjusted symbol: {} -> {}", ltx.symbol, symbols[&ltx.symbol]);
            ltx.symbol = symbols[&ltx.symbol].to_owned();
        }

        txs.push(ltx);
    }

    txs
}

/**
 * Reads the Cash Transaction records from the Flex Report.
 * Sorts by date/time, symbol, type.
 */
fn read_flex_report(cfg: &Config) -> Vec<CashTransaction> {
    let content = load_report(cfg);
    let response = FlexQueryResponse::from(content);

    let mut ib_txs = response
        .FlexStatements
        .FlexStatement
        .CashTransactions
        .CashTransaction;

    // txs.sort(key=operator.attrgetter("dateTime", "symbol", "type.name"))
    ib_txs.sort_unstable_by_key(|ct| {
        (
            ct.dateTime.to_owned(),
            ct.symbol.to_owned(),
            ct.r#type.to_owned(),
        )
    });

    ib_txs
}

fn compare_txs(ib_txs: Vec<CommonTransaction>, ledger_txs: Vec<CommonTransaction>) {
    for ibtx in ib_txs {
        let matches: Vec<&CommonTransaction> = if DATE_MODE == "effective" {
            todo!("complete");
        } else {
            ledger_txs
                .iter()
                .filter(|tx| {
                    tx.date == ibtx.date && tx.symbol == ibtx.symbol && tx.amount == ibtx.amount
                })
                .collect()
        };

        if matches.is_empty() {
            println!("New: {}", ibtx);
        }
    }
    println!("Complete.");
}

/**
 * Parameters for comparing the IB Flex report and Ledger report.
 */
#[derive(Debug)]
pub struct CompareParams {
    pub flex_report_path: Option<String>,
    pub flex_reports_dir: Option<String>,
    pub ledger_init_file: Option<String>,
}

// Tests

#[cfg(test)]
mod tests {
    use super::{compare, load_symbols};
    use crate::{
        compare::{CompareParams, convert_ib_txs},
        test_fixtures::*, flex_query_def::CashTransaction,
    };

    /// Load symbols through PriceDb.
    #[test]
    fn symbols_load_test() {
        let symbols = load_symbols().expect("symbols loaded");

        assert!(!symbols.is_empty());
    }

    #[rstest::rstest]
    #[test_log::test]
    fn test_convert_ib_txs(cash_transactions: Vec<CashTransaction>) {
        let ib_tx = convert_ib_txs(cash_transactions);

        assert!(!ib_tx.is_empty());
    }

    #[rstest::rstest]
    #[test_log::test]
    fn test_compare(cmp_params: CompareParams) {
        let actual = compare(cmp_params);

        assert!(!actual.is_err());
    }
}

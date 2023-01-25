/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::{collections::HashMap, ops::Mul, path::PathBuf};

use anyhow::{Error, Ok};
use as_symbols::SymbolMetadata;
use rust_decimal::Decimal;

use crate::{
    config::{get_cmp_config, Config},
    flex_enums::{cash_action, CashAction},
    flex_query::{CashTransaction, FlexQueryResponse},
    flex_reader::load_report,
    ledger_runner,
    model::CommonTransaction,
    ISO_DATE_FORMAT,
};

pub const TRANSACTION_DAYS: u8 = 60;
//pub(crate) const DATE_MODE: &str = "book"; // "book" / "effective"

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(params: CompareParams) -> anyhow::Result<String> {
    log::debug!("comparing distributions, params: {:?}", params);
    let cfg = get_cmp_config(&params);

    // get_ib_report_tx
    let ib_txs = get_ib_tx(&cfg);
    log::debug!("Found {} IB transactions", ib_txs.len());
    if ib_txs.len() == 0 {
        let msg = "No new IB transactions found. Exiting...\n";
        print!("{}", msg);
        return Ok(msg.into());
    }

    // get_ledger_tx
    let ledger_txs = ledger_runner::get_ledger_tx(cfg.ledger_init_file, params.effective_dates);
    log::debug!("Found {} Ledger transactions", ledger_txs.len());

    // compare
    let result = compare_txs(ib_txs, ledger_txs, params.effective_dates)?;

    Ok(result)
}

fn compare_txs(
    ib_txs: Vec<CommonTransaction>,
    ledger_txs: Vec<CommonTransaction>,
    use_effective_dates: bool,
) -> anyhow::Result<String> {
    let mut result = String::default();

    for ibtx in ib_txs {
        let ib_comparison_date = if use_effective_dates {
            ibtx.date.to_string()
        } else {
            // actual date
            ibtx.report_date.to_owned()
        };
        log::debug!("ib date: {:?}", ib_comparison_date);

        let matches: Vec<&CommonTransaction> = ledger_txs
            .iter()
            .filter(|tx| {
                // Compare:
                tx.date.date().format(ISO_DATE_FORMAT).to_string() == ib_comparison_date
                    && tx.symbol == ibtx.symbol
                    && tx.amount == ibtx.amount.mul(Decimal::NEGATIVE_ONE)
                    && tx.currency == ibtx.currency
                    && tx.r#type == ibtx.r#type
            })
            .collect();

        log::debug!("matches: {:?}", matches);

        if matches.is_empty() {
            let output = format!("New: {}\n", ibtx);
            print!("{}", output);
            result.push_str(&output);
        }
    }
    println!("Complete.");

    Ok(result)
}

/// Load the symbol mappings.
/// The resulting hashmap is <symbol, ledger symbol>.
fn load_symbols(path: &PathBuf) -> Result<HashMap<String, String>, Error> {
    log::debug!("loading symbols");

    let securities = as_symbols::read_symbols(path)
        .expect("Parsed symbols")
        .iter()
        .map(|sym| map_symbols(sym))
        .collect();
    Ok(securities)
}

/// Maps the SymbolMetadata into a hashmap of (ib_symbol, ledger_symbol) records.
fn map_symbols(symbol: &SymbolMetadata) -> (String, String) {
    (
        match &symbol.ib_symbol {
            Some(ib_sym) => ib_sym.to_owned(),
            None => symbol.symbol.to_owned(),
        },
        match &symbol.ledger_symbol {
            Some(ldg_sym) => ldg_sym.to_owned(),
            None => symbol.symbol.to_owned(),
        },
    )
}

/**
 * Returns transactions from the Flex Report, for comparison.
 * symbols is a HashMap of symbol rewrites.
 */
fn get_ib_tx(cfg: &Config) -> Vec<CommonTransaction> {
    let ib_txs = read_flex_report(cfg);

    convert_ib_txs(ib_txs, &cfg.symbols_path)
}

/// Converts IB CashTransaction XML record into a Common Transaction.
fn convert_ib_txs(ib_txs: Vec<CashTransaction>, symbols_path_str: &str) -> Vec<CommonTransaction> {
    // load symbols. Need a mapping to the ledger symbols for comparison.
    let symbols_path = PathBuf::from(symbols_path_str);
    log::debug!("loading symbols from {:?}", symbols_path);
    let symbols = load_symbols(&symbols_path).unwrap();
    log::debug!("symbols loaded: {:?}", symbols);

    let mut txs: Vec<CommonTransaction> = vec![];

    let to_include = [
        CashAction::WhTax.to_string(),
        CashAction::Dividend.to_string(),
    ];
    log::debug!("to include: {:?}", to_include);

    for tx in ib_txs {
        log::debug!("trying: {:?} {:?} ({:?})", tx.symbol, tx.r#type, cash_action(&tx.r#type));

        // skip any not matching the expected types.
        if !to_include.contains(&cash_action(&tx.r#type)) {
            println!("N/A: {}", tx);
            continue;
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
        .flex_statements
        .flex_statement
        .cash_transactions
        .cash_transaction;

    // txs.sort(key=operator.attrgetter("dateTime", "symbol", "type.name"))
    ib_txs.sort_unstable_by_key(|ct| {
        (
            ct.date_time.to_owned(),
            ct.symbol.to_owned(),
            ct.r#type.to_owned(),
        )
    });

    ib_txs
}

/**
 * Parameters for comparing the IB Flex report and Ledger report.
 */
#[derive(Debug)]
pub struct CompareParams {
    pub flex_report_path: Option<String>,
    pub flex_reports_dir: Option<String>,
    pub ledger_init_file: Option<String>,
    pub symbols_path: Option<String>,
    pub effective_dates: bool,
}

// Tests

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{compare, compare_txs, load_symbols};
    use crate::{
        compare::{convert_ib_txs, get_ib_tx, CompareParams},
        config::Config,
        flex_query::CashTransaction,
        ledger_runner::get_ledger_tx,
        test_fixtures::*,
    };

    /// Load symbols through PriceDb.
    #[test]
    fn symbols_load_test() {
        let symbol_path = PathBuf::from("tests/symbols.csv");

        let symbols = load_symbols(&symbol_path).expect("symbols loaded");

        assert!(!symbols.is_empty());
    }

    #[rstest::rstest]
    #[test_log::test]
    fn test_convert_ib_txs(cash_transactions: Vec<CashTransaction>) {
        let symbols_path = "tests/symbols.csv";

        let ib_tx = convert_ib_txs(cash_transactions, symbols_path);

        assert!(!ib_tx.is_empty());
    }

    #[rstest::rstest]
    #[test_log::test]
    fn test_compare_tx(cmp_config: Config) {
        let ib_txs = get_ib_tx(&cmp_config);
        let use_effective_dates = false;
        let ledger_txs = get_ledger_tx(cmp_config.ledger_init_file, use_effective_dates);

        log::debug!("comparing {:?} *** and *** {:?}", ib_txs, ledger_txs);

        let actual = compare_txs(ib_txs, ledger_txs, use_effective_dates);

        assert!(actual.is_ok());
        // assert!(false)
    }

    #[rstest::rstest]
    #[test_log::test]
    fn test_compare(cmp_params: CompareParams) {
        println!("comparing using: {:?}", cmp_params);

        let actual = compare(cmp_params);

        assert!(!actual.is_err());
    }

    /// tax adjustments come on one day and match several records in the past year.
    /// The report date needs to be matched to the effective date in this case,
    /// in addition to the transaction date/transaction date.
    #[test]
    fn test_compare_w_multiple_matches() {
        let cmp_params = CompareParams {
            flex_report_path: Some("tests/tax_adj_report.xml".into()),
            flex_reports_dir: None,
            ledger_init_file: Some("tests/tax_adj.ledgerrc".into()),
            symbols_path: Some("tests/symbols.csv".into()),
            effective_dates: false,
        };
        let actual = compare(cmp_params).unwrap();

        println!("result: {:?}", actual);

        let expected = r#"New: 2023-01-24/2022-04-01 BBN     WhTax    0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-01 BBN     WhTax   -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax    0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax   -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
Complete."#;

        /* These are in the ledger file:
        New: 2023-01-24/2022-03-01 BBN     WhTax    0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
        New: 2023-01-24/2022-03-01 BBN     WhTax   -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
         */

        assert_eq!(expected, actual);

        assert!(false);
    }
}

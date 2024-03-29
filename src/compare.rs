/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::{
    collections::HashMap,
    ops::Mul,
    path::{Path, PathBuf},
};

use anyhow::{Error, Ok};
use as_symbols::SymbolMetadata;
use rust_decimal::Decimal;

use crate::{
    flex_enums::{cash_action, CashAction},
    flex_query::{CashTransaction, FlexQueryResponse},
    flex_reader::load_report,
    ledger_runner::{self, get_ledger_start_date},
    model::CommonTransaction,
    ISO_DATE_FORMAT,
};

pub const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(params: CompareParams) -> anyhow::Result<String> {
    log::debug!("comparing distributions, params: {:?}", params);
    // let cfg = get_cmp_config(&params);

    // get_ib_report_tx
    let mut ib_txs = get_ib_tx(&params);
    log::debug!("Found {} IB transactions", ib_txs.len());
    if ib_txs.len() == 0 {
        let msg = "No new IB transactions found. Exiting...\n";
        print!("{}", msg);
        return Ok(msg.into());
    }

    // sort IB records by dates, symbol, type
    ib_txs.sort_unstable_by_key(|tx| {
        (
            tx.report_date.to_owned(),
            tx.date,
            tx.symbol.to_owned(),
            tx.r#type.to_owned(),
        )
    });
    log::debug!("sorted: {:?}", ib_txs);

    // identify the start date for the tx range:
    let start_date = get_oldest_ib_date(&ib_txs, params.effective_dates);

    // get_ledger_tx
    let ledger_txs = ledger_runner::get_ledger_tx(
        params.ledger_journal_file,
        start_date,
        params.effective_dates,
    );
    log::debug!("Found {} Ledger transactions", ledger_txs.len());

    // compare
    let result = compare_txs(ib_txs, ledger_txs, params.effective_dates)?;

    Ok(result)
}

fn compare_txs(
    ib_txs: Vec<CommonTransaction>,
    ledger_txs: Vec<CommonTransaction>,
    use_effective_date: bool,
) -> anyhow::Result<String> {
    let mut result = String::default();

    for ibtx in ib_txs {
        log::debug!(
            "Searching for matches for ib tx: {:?}\n among {:?}",
            ibtx,
            ledger_txs
        );

        let ib_comparison_date = get_comparison_date(&ibtx, use_effective_date);
        log::debug!("using ib date for comparison: {:?}", ib_comparison_date);

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
        
        // log::debug!("date: {:?}", ledger_txs[0].date.date().format(ISO_DATE_FORMAT).to_string());
        log::debug!("matching ledger txs: {:?}", matches);

        if matches.is_empty() {
            let output = format!("New: {}\n", ibtx);
            print!("{}", output);
            result.push_str(&output);
        }
    }
    println!("Complete.");

    Ok(result)
}

fn get_comparison_date(ibtx: &CommonTransaction, use_effective_date: bool) -> String {
    match use_effective_date {
        true => ibtx.date.format(ISO_DATE_FORMAT).to_string(),
        false => ibtx.report_date.to_owned(), // actual date
    }
}

/// Finds the date of the oldest transaction in the report.
/// This date is to be used for time-boxing Ledger report.
fn get_oldest_ib_date(ib_txs: &Vec<CommonTransaction>, use_effective_date: bool) -> String {
    if ib_txs.is_empty() {
        return get_ledger_start_date(None);
    }

    //ib_txs.sort_unstable_by_key(|ibtx| get_comparison_date(&ibtx, use_effective_date) );
    let oldest_date_record = ib_txs
        .iter()
        .min_by_key(|ibtx| get_comparison_date(&ibtx, use_effective_date))
        .expect("got oldest date");

    log::debug!("oldest tx: {:?}", oldest_date_record);

    get_comparison_date(&oldest_date_record, use_effective_date)
}

/// Load the symbol mappings.
/// The resulting hashmap is <symbol, ledger symbol>.
fn load_symbols(path: &PathBuf) -> Result<HashMap<String, String>, Error> {
    log::debug!("loading symbols from {:?}", path);

    // confirm the path exists
    let real_path = Path::new(path);
    if !real_path.exists() {
        panic!("The symbols file {:?} does not exist!", path);
    }

    let securities = as_symbols::read_symbols(path)
        .expect("Parsed symbols")
        .iter()
        .map(|sym| map_symbols(sym))
        .collect();
    Ok(securities)
}

/// Maps the SymbolMetadata into a hashmap of (ib_symbol, ledger_symbol) records.
fn map_symbols(meta: &SymbolMetadata) -> (String, String) {
    (
        match &meta.ib_symbol {
            Some(ib_sym) => ib_sym.to_owned(),
            None => {
                let Some(ref namespace) = meta.namespace
                    else { panic!("Missing namespace found!") };

                format!("{}:{}", namespace, meta.symbol)
            },
        },
        match &meta.ledger_symbol {
            Some(ldg_sym) => ldg_sym.to_owned(),
            None => meta.symbol.to_owned(),
        },
    )
}

/**
Returns transactions from the Flex Report, for comparison.
symbols is a HashMap of symbol rewrites.
*/
fn get_ib_tx(cfg: &CompareParams) -> Vec<CommonTransaction> {
    let ib_txs = read_flex_report(cfg);

    convert_ib_txs_into_common(ib_txs, cfg.symbols_path.as_str())
}

/// Converts IB CashTransaction XML record into a Common Transaction.
fn convert_ib_txs_into_common(ib_txs: Vec<CashTransaction>, symbols_path_str: &str) -> Vec<CommonTransaction> {
    // load symbols. Need a mapping to the ledger symbols for comparison.
    let symbols_path = PathBuf::from(symbols_path_str);
    let symbols = load_symbols(&symbols_path).unwrap();
    log::debug!("symbols loaded: {:?}", symbols);

    let mut txs: Vec<CommonTransaction> = vec![];

    let to_include = [
        CashAction::WhTax.to_string(),
        CashAction::Dividend.to_string(),
        CashAction::PaymentInLieu.to_string()
    ];
    log::debug!("to include: {:?}", to_include);

    for tx in ib_txs {
        log::debug!(
            "Converting ib tx: {:?} {:?} ({:?})",
            tx.symbol,
            tx.r#type,
            cash_action(&tx.r#type)
        );

        // skip any not matching the expected types.
        if !to_include.contains(&cash_action(&tx.r#type)) {
            log::debug!("Skipping. Wrong type ({:?})", &tx.r#type);
            println!("Skipped: {}", tx);
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
fn read_flex_report(cfg: &CompareParams) -> Vec<CashTransaction> {
    let content = load_report(cfg.flex_report_path.to_owned(), cfg.flex_reports_dir.to_owned());
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
    pub ledger_journal_file: Option<String>,
    pub symbols_path: String,
    pub effective_dates: bool,
}

// Tests

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{compare, load_symbols};
    use crate::{
        compare::{convert_ib_txs_into_common, CompareParams},
        flex_query::CashTransaction,
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

        let ib_tx = convert_ib_txs_into_common(cash_transactions, symbols_path);

        assert!(!ib_tx.is_empty());
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
    #[test_log::test]
    fn test_compare_w_multiple_matches() {
        let cmp_params = CompareParams {
            flex_report_path: Some("tests/tax_adj_report.xml".into()),
            flex_reports_dir: None,
            ledger_journal_file: Some("tests/tax_adj_journal.ledger".into()),
            symbols_path: "tests/symbols.csv".to_owned(),
            effective_dates: false,
        };
        let actual = compare(cmp_params).unwrap();

        println!("result: {:?}", actual);

        let expected = "";

        assert_eq!(expected, actual);
    }

    /// Same test but using effective dates.
    #[test_log::test]
    fn test_compare_w_multiple_matches_effective_dates() {
        let cmp_params = CompareParams {
            flex_report_path: Some("tests/tax_adj_report.xml".into()),
            flex_reports_dir: None,
            ledger_journal_file: Some("tests/tax_adj_journal.ledger".into()),
            symbols_path: "tests/symbols.csv".into(),
            effective_dates: true,
        };
        let actual = compare(cmp_params).unwrap();

        println!("result: {:?}", actual);

        let expected = r#"New: 2023-01-24/2022-04-01 BBN     WhTax       0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-01 BBN     WhTax      -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax       0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
New: 2023-01-24/2022-04-30 BBN     WhTax      -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
"#;

        /* These are in the ledger file:
        New: 2023-01-24/2022-03-01 BBN     WhTax    0.66 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
        New: 2023-01-24/2022-03-01 BBN     WhTax   -0.53 USD, BBN(US09248X1000) CASH DIVIDEND USD 0.1229 PER SHARE - US TAX
         */

        assert_eq!(expected, actual);
    }

    #[test_log::test]
    fn test_tcf() {
        let cmp_params = CompareParams {
            flex_report_path: Some("tests/tcf.xml".into()),
            flex_reports_dir: None,
            ledger_journal_file: Some("tests/tcf.ledger".into()),
            symbols_path: "tests/symbols.csv".into(),
            effective_dates: false,
        };
        let actual = compare(cmp_params).unwrap();

        let expected = "";

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_in_lieu() {
        let params = CompareParams {
            flex_report_path: Some("tests/in-lieu.xml".to_owned()),
            flex_reports_dir: None,
            ledger_journal_file: Some("tests/in-lieu.ledger".into()),
            symbols_path: "tests/symbols.csv".into(),
            effective_dates: false,
        };

        let actual = compare(params).unwrap();

        //let expected = r#"New: 2023-03-15/2023-03-16 EXXW    PaymentInLieu    3.74 EUR, EXXW(DE000A0H0744) PAYMENT IN LIEU OF DIVIDEND (Mixed Income)\n"#;
        let expected = "";
        
        assert_eq!(expected, actual);
    }

    #[test_log::test]
    fn test_same_symbols_different_exchange() {
        let cmp_params = CompareParams {
            flex_report_path: Some("tests/same_symbol.xml".into()),
            flex_reports_dir: None,
            ledger_journal_file: Some("tests/same_symbol.ledger".into()),
            symbols_path: "tests/symbols.csv".into(),
            effective_dates: false,
        };
        let actual = compare(cmp_params).unwrap();

//         let expected = r#"New: 2023-09-14/2023-09-15 ARCA:SDIV Dividend    5.04 USD, SDIV(US37960A6698) CASH DIVIDEND USD 0.21 PER SHARE (Ordinary Dividend)
// New: 2023-09-21/2023-09-22 BVME.ETF:SDIV    Dividend   10.26 USD, SDIV(IE00077FRP95) CASH DIVIDEND USD 0.09 PER SHARE (Mixed Income)"#;
        let expected = "";

        assert_eq!(expected, actual);
    }
}

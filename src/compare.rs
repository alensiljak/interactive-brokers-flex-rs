/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::{collections::HashMap, process::Command};

use anyhow::{Error, Ok};
use chrono::{Days, Local};

use crate::{
    flex_query_def::{CashTransaction, FlexQueryResponse},
    flex_query_reader::load_report,
    model::LedgerTransaction,
};

const DATE_MODE: &str = "book"; // "book" / "effective"
const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(params: CompareParams) -> anyhow::Result<()> {
    log::debug!("comparing distributions");

    // get_ib_report_tx
    let ib_tx = get_ib_tx(params.flex_report_path);

    // get_ledger_tx
    let ledger_tx = get_ledger_tx(params.ledger_init_file);

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

/// Get ledger transactions
/// Ledger must be callable from the current directory.
fn get_ledger_tx(ledger_init_file: Option<String>) -> Vec<LedgerTransaction> {
    let end_date = Local::now().date_naive();
    let start_date = end_date
        .checked_sub_days(Days::new(TRANSACTION_DAYS.into()))
        .expect("calculated start date");

    let command = r#"r -b {date_param} \
        -d \"(account =~ /income/ and account =~ /ib aus/) or \
        (account =~ /ib/ and account =~ /withh/)\""#;

    let date_param = start_date.format("%Y-%m-%d").to_string();
    let mut command = command.replace("{date_param}", date_param.as_str());

    if DATE_MODE == "effective" {
        command += " --effective"
    }

    if let Some(init_file) = ledger_init_file {
        command += format!("--init-file {}", &init_file).as_str();
    };

    log::debug!("running {}", command);

    let output = run_ledger(&command);

    log::debug!("ledger output: {:?}", output);

    // Parse output.
    let txs: Vec<LedgerTransaction> = output.iter().map(|line| parse_ledger_tx(line)).collect();

    txs
}

fn parse_ledger_tx(line: &String) -> LedgerTransaction {
    todo!("parse")
}

/// Runs Ledger with the given command and returns the output in lines.
fn run_ledger(cmd: &str) -> Vec<String> {
    let args: Vec<&str> = cmd.split(' ').collect();
    // remove the first attribute (ledger)
    let prog_args = &args[1..];

    let output = Command::new("ledger")
        .args(prog_args)
        .output()
        // .spawn()
        .expect("ledger command ran");

    let result: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.trim().to_owned())
        .collect();

    log::debug!("output is {:?}", result);

    result
}

/**
 * Returns transactions from the Flex Report, for comparison.
 * symbols is a HashMap of symbol rewrites.
 */
fn get_ib_tx(flex_report_path: Option<String>) -> Vec<LedgerTransaction> {
    // load symbols
    let symbols = load_symbols().unwrap();

    let ib_txs = read_flex_report(flex_report_path);

    convert_ib_tx(ib_txs)
}

fn convert_ib_tx(ib_txs: Vec<CashTransaction>) -> Vec<LedgerTransaction> {
    let mut txs: Vec<LedgerTransaction> = vec![];
    let skip = ["WHTAX", "DIVIDEND"];
    for tx in ib_txs {
        // todo: skip any not
        if skip.iter().any(|t| *t == tx.r#type) {
            println!("Skipping...");
        }

        let ltx: LedgerTransaction = (&tx).into();

        txs.push(ltx);
    }

    txs
}

/**
 * Reads the Cash Transaction records from the Flex Report.
 * Sorts by date/time, symbol, type.
 */
fn read_flex_report(flex_report_path: Option<String>) -> Vec<CashTransaction> {
    let content = load_report(flex_report_path);
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

fn compare_txs(ib_txs: Vec<LedgerTransaction>, ledger_txs: Vec<LedgerTransaction>) {
    for ibtx in ib_txs {
        let matches: Vec<&LedgerTransaction> = if DATE_MODE == "effective" {
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
pub struct CompareParams {
    pub flex_report_path: Option<String>,
    pub ledger_init_file: Option<String>,
}

impl CompareParams {
    pub fn new(flex_report_path: Option<String>, ledger_init_file: Option<String>) -> Self {
        Self {
            flex_report_path,
            ledger_init_file,
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::{compare, get_ledger_tx, load_symbols, run_ledger};
    use crate::{
        compare::{convert_ib_tx, CompareParams},
        flex_query_def::CashTransaction,
        test_fixtures::{cash_transactions, flex_report_path},
    };

    /// Load symbols through PriceDb.
    #[test]
    fn symbols_load_test() {
        let symbols = load_symbols().expect("symbols loaded");

        assert!(!symbols.is_empty());
    }

    /// Confirm that Ledger can be invoked from the current directory.
    #[test_log::test]
    fn run_ledger_test() {
        let cmd = "b active and cash";
        let actual = run_ledger(cmd);

        assert!(!actual.is_empty());
        assert_ne!(actual[0], String::default());
    }

    #[rstest::rstest]
    fn test_convert_ib_txs(cash_transactions: Vec<CashTransaction>) {
        let ib_tx = convert_ib_tx(cash_transactions);

        assert!(!ib_tx.is_empty());
    }

    /// Test fetching the required Ledger transactions.
    #[test_log::test]
    fn test_get_ledger_tx() {
        let actual = get_ledger_tx(None);

        assert!(!actual.is_empty());
    }

    #[rstest::rstest]
    fn test_comparison(flex_report_path: String) {
        let cmp_param = CompareParams {
            flex_report_path: Some(flex_report_path),
            ledger_init_file: None
        };
        let actual = compare(cmp_param);

        assert!(!actual.is_err());
    }
}

/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::{collections::HashMap, process::Command};

use anyhow::{Error, Ok};
use chrono::{Days, Local};

use crate::{
    flex_query_def::{CashTransaction, FlexQueryResponse},
    model::LedgerTransaction, flex_query_reader::load_report,
};

const DATE_MODE: &str = "book"; // "book" / "effective"
const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(flex_report_path: Option<String>) -> anyhow::Result<()> {
    log::debug!("comparing distributions");

    // get_ib_report_tx
    let ib_tx = get_ib_tx(flex_report_path);

    // get_ledger_tx
    let ledger_tx = get_ledger_tx();

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
fn get_ledger_tx() -> Vec<LedgerTransaction> {
    let end_date = Local::now().date_naive();
    let start_date = end_date
        .checked_sub_days(Days::new(TRANSACTION_DAYS.into()))
        .expect("calculated start date");

    let date_param = start_date.format("%Y-%m-%d").to_string();
    let command = r#"ledger r -b {date_param}
        -d \"(account =~ /income/ and account =~ /ib aus/) or
        (account =~ /ib/ and account =~ /withh/)\""#;
    let mut command = command.replace("{date_param}", date_param.as_str());

    if DATE_MODE == "effective" {
        command += " --effective"
    }

    let output = run_ledger(command);

    // Parse output.
    let txs: Vec<LedgerTransaction> = output.iter().map(|line| parse_ledger_tx(line)).collect();

    txs
}

fn parse_ledger_tx(line: &String) -> LedgerTransaction {
    todo!("parse")
}

/// Runs Ledger with the given command and returns the output in lines.
fn run_ledger(cmd: String) -> Vec<String> {
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
            //let output = format
            println!("New: {}", ibtx);
        }
    }
    println!("Complete.");
}

// Tests

#[cfg(test)]
mod tests {
    use super::{load_symbols, run_ledger, compare};
    use crate::{flex_query_def::CashTransaction, compare::convert_ib_tx, test_fixtures::{cash_transactions, flex_report_path}};

    /// Load symbols through PriceDb.
    #[test]
    fn symbols_load_test() {
        let symbols = load_symbols().expect("symbols loaded");

        assert!(!symbols.is_empty());
    }

    /// Confirm that Ledger can be invoked from the current directory.
    #[test_log::test]
    fn ledger_run_test() {
        let cmd = "ledger b active and cash".to_owned();
        let actual = run_ledger(cmd);

        assert!(!actual.is_empty());
        assert_ne!(actual[0], String::default());
    }

    #[rstest::rstest]
    fn test_convert_ib_txs(cash_transactions: Vec<CashTransaction>) {
        let ib_tx = convert_ib_tx(cash_transactions);

        assert!(!ib_tx.is_empty());
    }

    #[rstest::rstest]
    fn test_comparison(flex_report_path: String) {
        let actual = compare(Some(flex_report_path));

        assert!(!actual.is_err());
    }
}

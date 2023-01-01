/*!
 * Compares the downloaded Flex report with Ledger
 */

use std::{collections::HashMap, process::{Output, Command}};

use anyhow::{Error, Ok};
use chrono::{Days, Local};

use crate::{
    config::{get_cmp_config, Config},
    flex_query_def::{CashTransaction, FlexQueryResponse},
    flex_query_reader::load_report,
    ledger_reg_output_parser::{self},
    model::CommonTransaction,
};

const DATE_MODE: &str = "book"; // "book" / "effective"
const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare(params: CompareParams) -> anyhow::Result<()> {
    log::debug!("comparing distributions, params: {:?}", params);
    let cfg = get_cmp_config(&params);

    // get_ib_report_tx
    let ib_tx = get_ib_tx(&cfg);

    // get_ledger_tx
    let ledger_tx = get_ledger_tx(cfg.ledger_init_file);

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
fn get_ledger_tx(ledger_init_file: Option<String>) -> Vec<CommonTransaction> {
    let end_date = Local::now().date_naive();
    let start_date = end_date
        .checked_sub_days(Days::new(TRANSACTION_DAYS.into()))
        .expect("calculated start date");

    let date_param = start_date.format("%Y-%m-%d").to_string();

    // let cmd = get_ledger_cmd(&date_param, ledger_init_file);
    let args = get_ledger_args(&date_param, ledger_init_file);

    // log::debug!("running: {:?}", cmd);

    let mut output = run_ledger(args);

    // cleanup
    output = ledger_reg_output_parser::clean_up_register_output(output);

    // Parse output.
    let txs = ledger_reg_output_parser::get_rows_from_register(output);

    txs
}

#[allow(unused)]
fn get_ledger_args(date_param: &str, ledger_init_file: Option<String>) -> Vec<String> {
    let mut args: Vec<String> = vec!["r".to_owned()];
    args.push("-b".to_owned());
    args.push(date_param.to_owned());
    args.push("-d".to_owned());
    args.push(r#""(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)""#.to_owned());

    if DATE_MODE == "effective" {
        args.push("--effective".to_owned());
    }

    if let Some(init_file) = ledger_init_file {
        args.push("--init-file".to_owned());
        args.push(init_file.to_owned());
    }

    args
}

#[allow(unused)]
fn get_ledger_cmd(date_param: &str, ledger_init_file: Option<String>) -> String {
    let mut cmd = "r".to_string();

    cmd.push_str(" -b ");
    cmd.push_str(date_param);
    cmd.push_str(" -d");
    cmd.push_str(r#" "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)""#);

    if DATE_MODE == "effective" {
        cmd.push_str(" --effective");
    }

    if let Some(init_file) = ledger_init_file {
        cmd.push_str(" --init-file ");
        cmd.push_str(&init_file);
    };

    cmd
}

/// Runs Ledger with the given command and returns the output in lines.
/// cmd: The ledger command to execute, without `ledger` at the beginning.
fn run_ledger(args: Vec<String>) -> Vec<String> {
    // cmd: &str

    // let output = run_ledger_cmd(cmd);
    let output = run_ledger_args(args);

    log::debug!("output is {:?}", output);

    assert!(output.stderr.is_empty());

    let result: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        //.map(|line| line.trim().to_owned())
        .map(|line| line.to_owned())
        .collect();

    result
}

#[allow(unused)]
fn run_ledger_cmd(cmd: &str) -> Output {
    log::debug!("ledger cmd: {:?}", cmd);

    let output = Command::new("ledger")
        .arg(cmd)
        // .args(args)
        .output()
        // .spawn()
        .expect("ledger command ran");

    // let output = command!(r"ledger {cmd}")
    //     .expect("executed ledger")
    //     .output()
    //     .expect("ledger output");

    output
}

#[allow(unused)]
fn run_ledger_args(args: Vec<String>) -> Output {
    log::debug!("ledger args: {:?}", args);

    let output = Command::new("ledger")
    .args(args)
    .output()
    .expect("ledger command ran");

    output
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
    use super::{compare, get_ledger_tx, load_symbols, run_ledger};
    use crate::{
        compare::{convert_ib_txs, CompareParams},
        flex_query_def::CashTransaction,
        test_fixtures::*,
    };

    /// Load symbols through PriceDb.
    #[test]
    fn symbols_load_test() {
        let symbols = load_symbols().expect("symbols loaded");

        assert!(!symbols.is_empty());
    }

    /// Confirm that Ledger can be invoked from the current directory.
    #[rstest::rstest]
    #[test_log::test]
    fn run_ledger_test(ledger_init_path: String) {
        let mut cmd = "b active and cash --init-file ".to_string();
        cmd.push_str(&ledger_init_path);
        // let actual = run_ledger(&cmd);

        // let mut args = vec!["b".to_owned()];
        // args.push("active".to_owned());
        // args.push("and".to_owned());
        // args.push("cash".to_owned());
        // args.push("--init-file".to_owned());
        // args.push(ledger_init_path);
        let args = cmd.split_whitespace().into_iter()
            .map(|item| item.to_owned()).collect();
        let actual = run_ledger(args);

        assert!(!actual.is_empty());
        assert_ne!(actual[0], String::default());
        assert_eq!("           -3.00 EUR  Assets:Active:Cash", actual[0]);
    }

    /// Test fetching the required Ledger transactions.
    #[rstest::rstest]
    #[test_log::test]
    fn test_get_ledger_tx(ledger_init_path: String) {
        let path_opt = Some(ledger_init_path);
        let actual = get_ledger_tx(path_opt);

        assert!(!actual.is_empty());
        assert_eq!(7, actual.len());
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

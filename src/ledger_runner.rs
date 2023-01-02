/*!
 * Runs Ledger-cli to retrieve required reports.
 */

use std::process::{Command, Output};

use chrono::{Local, Days};

use crate::{model::CommonTransaction, ledger_reg_output_parser, compare::TRANSACTION_DAYS};

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

 #[allow(unused)]
fn get_ledger_args(date_param: &str, ledger_init_file: Option<String>) -> Vec<String> {
    let mut args: Vec<String> = vec!["r".to_owned()];
    args.push("-b".to_owned());
    args.push(date_param.to_owned());
    args.push("-d".to_owned());
    args.push(r#""(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)""#.to_owned());

    if crate::compare::DATE_MODE == "effective" {
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

    if crate::compare::DATE_MODE == "effective" {
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

/// Get ledger transactions
/// Ledger must be callable from the current directory.
pub fn get_ledger_tx(ledger_init_file: Option<String>) -> Vec<CommonTransaction> {
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

////
// Tests
////

#[cfg(test)]
mod tests {
    use crate::test_fixtures::*;
    use super::run_ledger;
    use super::get_ledger_tx;

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
}
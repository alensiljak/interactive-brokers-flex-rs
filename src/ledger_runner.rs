/*!
 * Runs Ledger-cli to retrieve required reports.
 */

use std::{
    process::{Command, Output},
};

use chrono::{Days, Local, NaiveDate};

use crate::{
    compare::TRANSACTION_DAYS, ledger_reg_output_parser, model::CommonTransaction, ISO_DATE_FORMAT,
};

/// Get ledger transactions
/// Ledger must be callable from the current directory.
pub fn get_ledger_tx(
    ledger_init_file: Option<String>,
    start_date: String,
    use_effective_dates: bool,
) -> Vec<CommonTransaction> {
    //let date_param = get_ledger_date_param(comparison_date);
    let date_param = start_date;

    let cmd = get_ledger_cmd(&date_param, ledger_init_file, use_effective_dates);

    log::debug!("running: {}", cmd);

    let output = cli_runner::run(&cmd);

    if !output.status.success() {
        let err = cli_runner::get_stderr(&output);
        panic!("Error running Ledger command: {}", err);
        // log::debug!("error: {:?}", err);
        // assert!(err.is_empty());
    }
    let out = cli_runner::get_stdout(&output);

    // log::debug!("ledger output: {:?}", out);

    let lines: Vec<&str> = out.lines().collect();

    // cleanup
    let clean_lines = ledger_reg_output_parser::clean_up_register_output(lines);

    // Parse output.
    let txs = ledger_reg_output_parser::get_rows_from_register(clean_lines);

    txs
}

/// Determines the starting date from which to take Ledger transactions.
/// This is one month from the comparison date.
pub fn get_ledger_start_date(comparison_date: Option<String>) -> String {
    let end_date = match &comparison_date {
        Some(date_value) => {
            NaiveDate::parse_from_str(&date_value, ISO_DATE_FORMAT).expect("correct date")
        }
        None => Local::now().date_naive(),
    };

    let start_date = end_date
        .checked_sub_days(Days::new(TRANSACTION_DAYS.into()))
        .expect("calculated start date");
    let date_param = start_date.format(ISO_DATE_FORMAT).to_string();

    log::debug!("Ledger start date: {:?} -> {:?}", comparison_date, date_param);

    date_param
}

fn run_ledger_args(args: Vec<String>) -> Output {
    log::debug!("ledger args: {:?}", args);

    let output = Command::new("ledger")
        .args(args)
        .output()
        .expect("ledger command ran");

    output
}

/// Assemble the Ledger query command.
fn get_ledger_cmd(
    start_date: &str,
    ledger_init_file: Option<String>,
    effective_dates: bool,
) -> String {
    let mut cmd = format!("ledger r -b {start_date} -d");

    cmd.push_str(r#" "(account =~ /income/ and account =~ /ib/) or"#);
    cmd.push_str(r#" (account =~ /expenses/ and account =~ /ib/ and account =~ /withh/)""#);

    if effective_dates {
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
/// Returns the lines of the Ledger output.
#[allow(unused)]
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

////
// Tests
////

#[cfg(test)]
mod tests {
    use super::get_ledger_tx;
    use super::run_ledger;
    use crate::ledger_runner::get_ledger_start_date;
    use crate::test_fixtures::*;

    /// Confirm that Ledger can be invoked from the current directory.
    #[rstest::rstest]
    #[test_log::test]
    fn run_ledger_test(ledger_init_path: String) {
        let mut cmd = "b active and cash --init-file ".to_string();
        cmd.push_str(&ledger_init_path);

        let args = cmd
            .split_whitespace()
            .into_iter()
            .map(|item| item.to_owned())
            .collect();
        let actual = run_ledger(args);

        assert!(!actual.is_empty());
        assert_ne!(actual[0], String::default());
        assert_eq!("           -3.00 EUR  Assets:Active:Cash", actual[0]);
    }

    /// Test fetching the required Ledger transactions.
    #[rstest::rstest]
    #[test_log::test]
    fn test_get_ledger_tx(ledger_init_path: String) {
        println!("ledger_init_path: {:?}", ledger_init_path);

        let path_opt = Some(ledger_init_path);
        let start_date = get_ledger_start_date(None);
        let actual = get_ledger_tx(path_opt, start_date, false);

        println!("txs: {:?}", actual);

        assert!(!actual.is_empty());
        assert_eq!(2, actual.len());
    }

    /// Run the complex query on Ledger, using shell-words.
    #[test_log::test]
    fn test_ledger_words() {
        let cmd = r#"r -b 2022-03-01 -d  "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
        let args = shell_words::split(cmd).unwrap();

        let actual = run_ledger(args);

        let expected: Vec<&str> = r#"2022-12-15 TRET_AS Distribution               Income:Investment:IB:TRET_AS                      -38.40 EUR           -38.40 EUR
                                              Expenses:Investment:IB:Withholding Tax              5.77 EUR           -32.63 EUR"#.lines().collect();

        assert!(!actual.is_empty());
        assert_eq!(expected, actual);
    }

    #[test_log::test]
    fn test_shellwords() {
        let cmd = r#"ledger r -b 2022-03-01 -d "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
        let actual = shell_words::split(cmd).unwrap();

        log::debug!("result: {:?}", actual);

        assert!(!actual.is_empty());
        assert_eq!(8, actual.len());
    }

    #[test_log::test]
    fn test_cli_runner() {
        let cmd = r#"ledger r -b 2022-03-01 -d "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;

        let actual = cli_runner::run(cmd);

        let stderr: String = String::from_utf8(actual.stderr).expect("stdout string");
        let stdout: String = String::from_utf8(actual.stdout).expect("stdout string");

        assert!(!stdout.is_empty());
        assert!(stderr.is_empty());
    }

    #[test_log::test]
    fn test_output_conversion() {
        let cmd = r#"ledger r -b 2022-03-01 -d "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
        let output = cli_runner::run(cmd);

        let so = cli_runner::get_stdout(&output);
        log::debug!("so: {:?}", so);
        assert!(!so.is_empty());

        let se = cli_runner::get_stderr(&output);
        assert!(se.is_empty());
    }
}

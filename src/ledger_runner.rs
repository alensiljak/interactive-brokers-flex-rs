/*!
 * Runs Ledger-cli to retrieve required reports.
 */

use std::process::{Command, Output};

use chrono::{Days, Local};

use crate::{compare::TRANSACTION_DAYS, ledger_reg_output_parser, model::CommonTransaction};

/// Get ledger transactions
/// Ledger must be callable from the current directory.
pub fn get_ledger_tx(ledger_init_file: Option<String>) -> Vec<CommonTransaction> {
    let end_date = Local::now().date_naive();
    let start_date = end_date
        .checked_sub_days(Days::new(TRANSACTION_DAYS.into()))
        .expect("calculated start date");

    let date_param = start_date.format("%Y-%m-%d").to_string();

    let cmd = get_ledger_cmd(&date_param, ledger_init_file);

    log::debug!("running: {}", cmd);

    // let output = run_ledger(args);
    let output = cli_runner::run(&cmd);
    let out = cli_runner::get_stdout(&output);

    let err = cli_runner::get_stderr(&output);
    log::debug!("error: {:?}", err);
    assert!(err.is_empty());

    log::debug!("ledger output: {:?}", out);

    let lines: Vec<&str> = out.lines().collect();
    // let lines = output.iter().map(|item| item.as_str()).collect();

    // cleanup
    let clean_lines = ledger_reg_output_parser::clean_up_register_output(lines);

    // Parse output.
    let txs = ledger_reg_output_parser::get_rows_from_register(clean_lines);

    txs
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

fn get_ledger_cmd(date_param: &str, ledger_init_file: Option<String>) -> String {
    let mut cmd = format!("ledger r -b {date_param} -d");

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
    use crate::test_fixtures::*;

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
        let path_opt = Some(ledger_init_path);
        let actual = get_ledger_tx(path_opt);

        log::debug!("txs: {:?}", actual);

        assert!(!actual.is_empty());
        assert_eq!(2, actual.len());
    }

    /// Run the complex query on Ledger, using shell-words.
    #[test_log::test]
    fn test_ledger_words() {
        let cmd = r#"r -b 2022-03-01 -d  "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
        let args = shell_words::split(cmd).unwrap();

        let actual = run_ledger(args);

        let expected: Vec<&str> = r#"2022-12-15 TRET Distribution                  Income:Investment:IB:TRET_AS                      -38.40 EUR           -38.40 EUR
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

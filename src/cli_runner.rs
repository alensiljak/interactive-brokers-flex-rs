/*!
 * CLI-app runner.
 */

use std::process::{Command, Output};

/// Runs a CLI application.
///
/// Runs a given command with parameters. The command is given as &str.
///
/// Returns the output of the CLI application as a string.
///
/// Example:
/// ```
/// let cmd = r#"ledger r -b 2022-03-01 -d  "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
/// let output = cli_runner::run(cmd);
/// assert!(output.error.is_empty());
/// ```
pub fn run(command: &str) -> Output {
    // parse attributes
    let args = shell_words::split(command).expect("command arguments parsed");

    // the first argument is the application name
    let program = &args[0];
    let prog_args = &args[1..];

    Command::new(program)
        .args(prog_args)
        .output()
        .expect("CLI output")
    // CliOutput::new(output)
}

pub fn get_stdout<'a>(output: &'a Output) -> &'a str {
    core::str::from_utf8(&output.stdout).expect("stdout as &str")
    // String::from_utf8(output.stdout).expect("stdout as string")
}

pub fn get_stderr<'a>(output: &'a Output) -> &'a str {
    core::str::from_utf8(&output.stderr).expect("stdout as &str")
    // String::from_utf8(output.stderr).expect("stderr as string")
}

#[cfg(test)]
mod tests {
    use super::{get_stdout, get_stderr};

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

        let actual = super::run(cmd);

        let stderr: String = String::from_utf8(actual.stderr).expect("stdout string");
        let stdout: String = String::from_utf8(actual.stdout).expect("stdout string");

        assert!(!stdout.is_empty());
        assert!(stderr.is_empty());
    }

    #[test]
    fn test_output_conversion() {
        let cmd = r#"ledger r -b 2022-03-01 -d "(account =~ /income/ and account =~ /ib/) or (account =~ /ib/ and account =~ /withh/)" --init-file tests/init.ledger"#;
        let output = super::run(cmd);

        let so = get_stdout(&output);
        assert!(!so.is_empty());

        let se = get_stderr(&output);
        assert!(se.is_empty());
    }
}

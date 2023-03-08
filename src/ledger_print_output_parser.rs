use std::ops::Index;

use chrono::{NaiveDate, ParseError};

use crate::{model::CommonTransaction, ISO_DATE_FORMAT};

/**
Parses the output of the `ledger print` command.
This is the regular Ledger syntax.
*/
pub(crate) fn parse_print_output(lines: Vec<&str>) -> Vec<CommonTransaction> {
    let mut tx: Option<CommonTransaction> = None;

    for line in lines {
        // lines to ignore
        if line.starts_with(';') {
            continue;
        }
        if line == "" {
            continue;
        }

        // data lines
        tx = parse_tx_row(line);

        if tx.is_some() {
            // got the (new) transaction header.
            continue;
        }

        // otherwise parse the postings.
        let posting = parse_posting_row(line);

        //todo!("merge posting and tx");

        println!("line {}", line);
    }

    todo!("complete")
}

fn parse_tx_row(line: &str) -> Option<CommonTransaction> {
    // date
    // always use ISO dates, sorry
    let date = get_date_from_line(line);
    if date.is_none() {
        return None;
    }

    let payee_start_index = 11;
    let comment_index = line.find(';');
    let payee_end_index = match comment_index {
        Some(end_index) => end_index,
        None => line.len(),
    };

    // payee
    let payee = &line[payee_start_index..payee_end_index];

    let mut tx = CommonTransaction::default();
    // tx.date = date.unwrap();

    todo!("parse tx header")
}

fn get_date_from_line(line: &str) -> Option<NaiveDate> {
    let date_string = &line[0..10];
    let result = NaiveDate::parse_from_str(&date_string, ISO_DATE_FORMAT);
    match result {
        Ok(date) => Some(date),
        Err(_) => None,
    }
}

fn parse_posting_row(line: &str) -> CommonTransaction {
    todo!("parse posting")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::ledger_print_output_parser::parse_print_output;

    #[test]
    fn parse_print() {
        let journal = fs::read_to_string("tests/journal.ledger").expect("test file read");
        let lines = journal.lines().collect();

        let actual = parse_print_output(lines);

        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());
    }
}

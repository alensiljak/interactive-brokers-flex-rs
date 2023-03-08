use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{model::CommonTransaction, ISO_DATE_FORMAT};

/**
Parses the output of the `ledger print` command.
This is the regular Ledger syntax.
*/
pub(crate) fn parse_print_output(lines: Vec<&str>) -> Vec<CommonTransaction> {
    let mut tx = CommonTransaction::default();
    let mut result: Vec<CommonTransaction> = vec![];

    for line in lines {
        let trimmed = line.trim();

        // lines to ignore
        if trimmed.starts_with(';') {
            continue;
        }
        if trimmed == "" {
            continue;
        }

        // data lines
        let have_tx = parse_tx_row(trimmed);

        if have_tx.is_some() {
            // got the (new) transaction header.
            tx = have_tx.unwrap();
            continue;
        }

        // otherwise parse the postings.
        let mut posting = parse_posting_row(trimmed);

        // merge posting and tx header
        posting.date = tx.date;
        posting.payee = tx.payee.to_owned();

        // println!("line {}", line);

        result.push(posting);
    }

    result
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
    tx.date = date.unwrap().and_hms_opt(0, 0, 0).unwrap();
    tx.payee = payee.to_owned();

    Some(tx)
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
    let mut tx = CommonTransaction::default();

    let have_amount = line.find("  ");

    // let parts: Vec<&str> = line.split("  ").collect();
    tx.account = match have_amount {
        Some(split_index) => line[0..split_index].to_owned(),
        None => line.to_owned(),
    };

    // is there an amount?
    if have_amount.is_some() {
        let amount_index = have_amount.unwrap();
        let mut amount_str = &line[amount_index..];
        // log::debug!("starting amount parse from '{:0}'", amount_str);
        amount_str = amount_str.trim();
        log::debug!("amount string is '{:0}'", amount_str);

        // split the currency
        let amount_parts: Vec<&str> = amount_str.split(' ').collect();

        tx.amount = Decimal::from_str(amount_parts[0]).expect("amount value");
        tx.currency = amount_parts[1].to_owned();
    }

    tx
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::ledger_print_output_parser::parse_print_output;

    #[test_log::test]
    fn parse_print() {
        let journal = fs::read_to_string("tests/journal.ledger").expect("test file read");
        let lines = journal.lines().collect();

        let actual = parse_print_output(lines);

        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());
    }
}

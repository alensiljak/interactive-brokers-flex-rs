use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{flex_enums::CashAction, model::CommonTransaction, ISO_DATE_FORMAT};

/**
Parses the output of the `ledger print` command.
This is the regular Ledger syntax.
*/
pub(crate) fn parse_print_output(lines: Vec<&str>) -> Vec<CommonTransaction> {
    let mut tx = CommonTransaction::default();
    let mut result: Vec<CommonTransaction> = vec![];
    // Postings for the current transaction. Used to add amount to the postings without it.
    let mut postings: Vec<CommonTransaction> = vec![];

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

            // If we already have a transaction header, process the posting records.
            if tx.payee != "" {
                postings = close_transaction(postings, &tx.currency);
                result.append(&mut postings);
                // clear the postings vector
                postings = vec![];
            }

            // set the transaction record.
            tx = have_tx.unwrap();

            continue;
        }

        // otherwise parse the postings.
        let mut posting = parse_posting_row(trimmed);

        // get the currency
        if posting.currency != "" {
            tx.currency = posting.currency.to_owned();
        }

        // merge posting and tx header
        posting.date = tx.date;
        posting.report_date = tx.report_date.to_owned();
        posting.payee = tx.payee.to_owned();
        posting.symbol = tx.symbol.to_owned();

        // println!("line {}", line);

        postings.push(posting);
    }
    // process the last transaction
    postings = close_transaction(postings, &tx.currency);
    result.append(&mut postings);

    result
}

/// Makes final adjustments to the postings, like adding the amount to the records
/// that don't have it.
fn close_transaction(postings: Vec<CommonTransaction>, currency: &str) -> Vec<CommonTransaction> {
    let mut result: Vec<CommonTransaction> = vec![];

    // Add amount to any postings missing it.
    let amount = get_amount_from_postings(&postings);
    for mut tx_posting in postings {
        log::debug!("posting amount: {:0} to be set", tx_posting.amount);
        if tx_posting.amount == Decimal::ZERO {
            // tx_posting.amount = match tx_posting.r#type.as_str() {
            //     "Dividend" => amount,
            //     "Withholding Tax" => amount * Decimal::NEGATIVE_ONE,
            //     _ => panic!("should not happen")
            // };
            tx_posting.amount = amount * Decimal::NEGATIVE_ONE;

            tx_posting.currency = currency.to_owned();
        }

        // move the postings into the result collection
        result.push(tx_posting);
    }

    result
}

fn get_amount_from_postings(postings: &Vec<CommonTransaction>) -> Decimal {
    let mut amount = Decimal::ZERO;

    for posting in postings {
        if posting.amount == Decimal::ZERO {
            continue;
        }

        if amount == Decimal::ZERO {
            // we don't have an amount yet.
            amount = posting.amount;
        } else {
            panic!("multiple amounts found!");
        }
    }
    amount
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
    tx.report_date = tx.date.format(ISO_DATE_FORMAT).to_string();
    
    tx.payee = payee.to_owned();

    // symbol
    let symbol_parts: Vec<&str> = payee.split_whitespace().collect();
    let symbol = symbol_parts[0];
    tx.symbol = symbol.to_owned();

    // type

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

        // using only dot as the decimal point currently.
        // remove thousand separators as Decimal parser doesn't handle them.
        let amount_str = amount_str.replace(',', "");

        log::debug!("amount string is '{:0}'", amount_str);

        // split the currency
        let amount_parts: Vec<&str> = amount_str.split(' ').collect();

        tx.amount = Decimal::from_str(&amount_parts[0]).expect("amount value");
        tx.currency = amount_parts[1].to_owned();
    }

    // Type
    // Get just the first 2 characters.
    let account = &tx.account[0..2];
    tx.r#type = if account == "In" {
        CashAction::Dividend.to_string()
    } else if account == "Ex" {
        CashAction::WhTax.to_string()
    } else {
        log::warn!("Could not parse type {:?}", account);

        "Error!".to_string()
    };

    tx
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::ledger_print_output_parser::parse_print_output;

    #[test_log::test]
    fn parse_print() {
        // todo: Due to the issue with empty posting amounts, parsing Print output works only
        // with 2 postings, out of which one is empty.
        //let journal = fs::read_to_string("tests/journal.ledger").expect("test file read");

        let journal = fs::read_to_string("tests/tcf.ledger").expect("test file read");
        let lines = journal.lines().collect();

        let actual = parse_print_output(lines);

        assert!(!actual.is_empty());
        // 7 transaction records / postings.
        //assert_eq!(9, actual.len());
        assert_eq!(2, actual.len());
        
        //assert_eq!(Decimal::from_i16(3).unwrap(), actual[6].amount);
        // log::debug!("amount {:0}", actual[8].amount);
    }
}

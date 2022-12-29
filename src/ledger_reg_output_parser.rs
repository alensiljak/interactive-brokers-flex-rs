/*!
 * Parser for Ledger's output of the `register` command.
 */

use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::{model::CommonTransaction, ISO_DATE_FORMAT};

/**
 * Ledger Register row.
 */
pub struct RegisterRow {

}

/**
 * Clean-up the ledger register report.
 * The report variable is a list of lines.
 */
pub fn clean_up_register_output(lines: Vec<String>) -> Vec<String> {
    let mut new_vec = vec![];

    // eliminate useless lines
    for line in lines {
        if line.is_empty() {
            continue;
        }

        // Check the account line. If empty, skip. This is just the running total.
        if line.chars().nth(50).unwrap() == ' ' {
            continue;
        }

        new_vec.push(line);
    }

    new_vec
}

/**
 * Parse raw lines from the ledger register output and get RegisterRow.
 */
pub fn get_rows_from_register(ledger_lines: Vec<String>) -> Vec<CommonTransaction> {
    let mut txs: Vec<CommonTransaction> = vec![];
    // remember the header row, which contains the medatada: date, symbol.
    let prev_row = CommonTransaction::default();

    for line in ledger_lines {
        let tx = get_row_from_register_line(&line, &prev_row);

        // todo:
        //if tx.date
        // prev_row = tx

        txs.push(tx);
    }
    txs
}

/// Parse one register line into a Transaction object
fn get_row_from_register_line(line: &str, header: &CommonTransaction) -> CommonTransaction {
    // header is the transaction with the date (and other metadata?)

    log::debug!("parsing: {:?}", line);

    if line.is_empty() {
        panic!("The lines must be prepared by `clean_up_register_output`");
    }

    let has_symbol = line.chars().nth(1).unwrap() != ' ';

    let date_str = &line[0..10].trim();
    let payee_str = &line[11..46].trim();
    let account_str = &line[46..85].trim();
    let amount_str = &line[85..107].trim();

    let mut tx = CommonTransaction::default();

    // Date
    tx.date = if date_str.is_empty() {
        header.date
    } else {
        // parse
        log::debug!("parsing date: {:?}", date_str);

        let tx_date = NaiveDate::parse_from_str(date_str, ISO_DATE_FORMAT)
            .expect("valid date expected");
        NaiveDateTime::from(tx_date.and_hms_opt(0, 0, 0).unwrap())
    };

    // Payee
    tx.payee = if payee_str.is_empty() {
        header.payee.to_owned()
    } else {
        payee_str.to_string()
    };

    // Symbol
    tx.symbol = if has_symbol {
        let parts: Vec<&str> = payee_str.split_whitespace().collect();
        let mut symbol = parts[0];
        if symbol.contains('.') {
            let index = symbol.find('.').unwrap();
            symbol = &symbol[0..index];
        }
        symbol.to_string()
    } else {
        header.symbol.to_string()
    };

    // Type
    // Get just the first 2 characters.
    let account = &account_str[0..3];
    tx.r#type = if account == "In" {
        "Dividend".to_string()
    } else if account == "Ex" {
        "Tax".to_string()
    } else {
        "Error!".to_string()
    };

    // Account
    tx.account = account_str.to_string();

    // Amount
    // Get from the end.
    let parts: Vec<&str> = amount_str.split_whitespace().collect();
    if parts.len() > 2 {
        println!("cannot parse: {:?}", tx);
    }
    assert!(parts.len() == 2);

    let amount = parts[0].replace(",", "");
    tx.amount = Decimal::from_str(&amount).expect("a valid number");

    // Currency
    tx.currency = parts[1].to_string();

    tx
}

// tests

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use rust_decimal::Decimal;

    use crate::model::CommonTransaction;

    use super::get_row_from_register_line;

    /**
     * Parse the transaction top row, with date/payee/account/amount
     * `l r --init-file tests/init.ledger`
     */
    #[test_log::test]
    fn parse_header_row() {
        let line = r#"2022-12-01 Supermarket                        Expenses:Food                                      15.00 EUR            15.00 EUR"#;

        let header = CommonTransaction::default();

        let actual = get_row_from_register_line(line, &header);

        log::debug!("actual: {:?}", actual);

        // Assertions

        // Date
        assert_eq!(actual.date.year(), 2022);
        // Payee
        assert!(!actual.payee.is_empty());
        assert_eq!(actual.payee, "Supermarket");
        // Account
        assert!(!actual.account.is_empty());
        assert_eq!(actual.account, "Expenses:Food");
        // Type
        assert!(!actual.r#type.is_empty());
        // Amount
        assert!(!actual.amount.is_zero());
        assert_eq!(actual.amount, Decimal::from(15));
        // Currency
        assert!(!actual.currency.is_empty());
        assert_eq!(actual.currency, "EUR");
    }

    /**
     * Parse the posting rows (not the top row).
     * `l r --init-file tests/init.ledger`
     */
    #[test]
    fn parse_posting_row() {
        let header = CommonTransaction::default();
        
        let line = r#"                                              Assets:Bank:Checking                              -15.00 EUR                    0"#;

        let actual = get_row_from_register_line(line, &header);

        assert!(!actual.amount.is_zero());
    }
}
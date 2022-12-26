/*
 * Compare the downloaded report with Ledger
 */

use std::collections::HashMap;

use anyhow::{Error, Ok};
use pricedb::model::Security;

const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub fn compare() {
    log::debug!("comparing distributions");

    // load symbols
    let symbols = load_symbols();

    todo!("get_ledger_tx");
    todo!("get_ib_report_tx");
    todo!("compare");
}

/// Load symbol mappings from PriceDb
fn load_symbols() -> Result<HashMap<String, String>, Error> {
    log::debug!("loading symbols");

    // read symbols from pricedb
    let cfg = pricedb::load_config();
    let pdb = pricedb::App::new(cfg);
    // get all the securities that have a different symbol in ledger.
    let securities: Vec<Security> = pdb.get_dal().get_securities(None)
        .into_iter().filter(|sec| sec.ledger_symbol.is_some())
        .collect();

    let mut result: HashMap<String, String> = HashMap::new();
    for sec in securities {
        result.insert(sec.symbol, sec.ledger_symbol.unwrap());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::load_symbols;

    #[test]
    fn symbols_load_test() {
        let symbols = load_symbols().expect("symbols loaded");

        assert!(!symbols.is_empty());
    }
}
/*
 * Compare the downloaded report with Ledger
 */

use anyhow::{Error, Ok};
use sqlx::{SqliteConnection, Connection};

use crate::model::Price;

const TRANSACTION_DAYS: u8 = 60;

/**
 * Compares transactions in the downloaded IB Flex report to Ledger.
 */
pub async fn compare() {
    log::debug!("comparing prices");
    
    // load symbols
    let symbols = load_symbols().await;

    todo!("get_ledger_tx");
    todo!("get_ib_report_tx");
    todo!("compare");
}

/// Load symbol mappings from PriceDb
async fn load_symbols() -> Result<(), Error> {
    log::debug!("loading symbols");

    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    let prices: Vec<Price> = sqlx::query_as::<_, Price>("select * from prices")
        .fetch_all(&mut conn).await?;

    for price in prices {
        println!("price: {:?}", price);
    }

    Ok(())
}
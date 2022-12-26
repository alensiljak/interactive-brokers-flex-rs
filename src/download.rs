use chrono::Local;

use crate::{config::Config, APP_NAME};

/**
 * The module for download of IB Flex reports.
 */
pub fn download() {
    let today_date = Local::now().date_naive();
    let today = today_date.format("%Y-%m-%d");
    let filename = format!("{today}_cash-tx.xml");

    let cfg = crate::read_config();

    todo!("download report");
}


#[cfg(test)]
mod tests {
    // use crate::{config::Config, download::read_config};

    // #[test]
    // fn read_config_test() {
    //     let cfg = read_config();

    //     assert_ne!(cfg, Config::default());
    // }
}
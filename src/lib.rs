/*!
 * Interactive Brokers Flex Query library
 * 
 * Project documentation: <https://github.com/alensiljak/interactive-brokers-flex-rs>
 */

pub mod compare;
pub mod download;
pub mod flex_query;
pub mod flex_reader;
pub mod flex_statement;
pub mod ledger_print_output_parser;
pub mod ledger_reg_output_parser;
pub mod ledger_runner;
pub mod model;
mod flex_enums;
#[cfg(test)]
mod test_fixtures;

pub const ISO_DATE_FORMAT: &str ="%Y-%m-%d";

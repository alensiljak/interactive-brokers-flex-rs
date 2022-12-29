/*!
 * Interactive Brokers Flex Query library
 * 
 * Project documentation: <https://github.com/alensiljak/interactive-brokers-flex-rs>
 */

pub mod compare;
pub mod config;
pub mod download;
pub mod flex_query_def;
pub mod flex_query_reader;
pub mod flex_statement;
pub mod model;
#[cfg(test)]
mod test_fixtures;

pub const ISO_DATE_FORMAT: &str ="%Y-%m-%d";

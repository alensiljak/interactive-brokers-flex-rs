# interactive-brokers-flex-rs
Tools to assist with IB Flex reports and Ledger

The crate is incomplete but there are funcioning parts. These can be seen in Tests.

This crate contains a CLI and a library which assist with Flex Reports from Interactive Brokers.
There are several components involved:

- CLI `ibflex` uses the `ibflex` library
- `ibflex` library exposes the following functionality
  - downloans the Flex report (incomplete)
  - runs the comparison for the downloaded Cash Transactions Flex .xml report and the last 60 days in Ledger (incomplete)
  - parses IB Flex report
  - runs Ledger command to retrieve the distribution transactions in the last 60 days (incomplete)
- `pricedb` crate ([repo](https://github.com/alensiljak/pricedb-rust)) uses a database which contains the list of securities with the Symbol mapping between IB Flex report and Ledger. I.e. symbol `VHYL` in the report is `VHYL_AS` in Ledger. The package is a dependency of `ibflex` but must be configured manually.

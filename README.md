# interactive-brokers-flex-rs
Tools to assist with IB Flex reports and Ledger-cli comparison

This crate contains a CLI application and is also a library which assists working with Flex Reports from Interactive Brokers. It simplifies the Flex Query download and compares the downloaded transactions (distributions and tax) to the records in Ledger, identifying missing ones.

There are several components in the package:

- The command-line application (CLI) `ibflex`, provides all the features of the `ibflex` library
- `ibflex` library exposes the following functionality:
  - downloads the IB Flex Query report
  - parses IB Flex Query report
  - runs Ledger-cli to retrieve the transactions in the last 60 days
  - compares the Cash Transactions from the downloaded Flex Query .xml report to the Ledger transactions
- `as-symbols` [crate](https://crates.io/crates/as-symbols) provides the Symbol mapping between IB Flex report and Ledger. I.e. symbol `VHYL` in the report is `VHYL_AS` in Ledger.

The project started as a rewrite of my Python scripts and is intended to be expanded as needed, to parse Flex Queries.

# Configuration

To view the current configuration, run

```
ibflex cfg
```

The config file will be created automatically if it does not exist.

To edit the values, use any text editor.

## Symbols Configuration

See `as-symbols` [crate](https://crates.io/crates/as-symbols) for instructions on how to set up the symbols data file.
At the moment this is required for the symbol mapping. The symbols in IB (i.e. VHYL) may be mapped to a different symbol in Ledger (i.e. VHYL_AS).

# Usage

Downloading the Flex Query report requires Query Id and the Token. These can be passed in several ways:

1) as parameters to the download (Dl) method. Use parameters `--queryid` and `--token`.
2) as environment variables:

  The application will read the values from environment variables:

  - `IBFLEX_TOKEN`
  - `IBFLEX_QUERYID`

3) in the configuration file: The application will read the configuration file `ibflex.toml`, which is located in the current directory. See the section below.

## Download

The required parameters for downloading the Flex report are:

- `flex_query_id`
- `ib_token`

Once this is set up, invoke the CLI:

```
ifblex dl
```

This will save the report in the current directory. The filename will contain today's date.

## Comparison

To compare the transactions, run

```
ibflex cmp
```

This will compare the downloaded IB transactions to the transactions in Ledger. The new Dividend and Tax transactions will be reported as New. The other transactions will be reported as Skipped.

# Changelog

See [Changelog](CHANGELOG.md)

# Credits

- [ibflex](https://github.com/csingley/ibflex) Python library

# License

See [LICENSE](LICENSE) file.

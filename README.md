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

# Usage

Prior to running, you may need to configure the required parameters in the configuration file `ibflex.toml`, which is located in the current directory.

## Configuration

To view the current configuration, run

```
ibflex cfg
```

This will display the current parameters. 

If the file does not exist, it will be automatically created by this command.

To edit the values, use any text editor.

## Download

The required parameters for downloading the Flex report are:

- `flex_query_id`
- `ib_token`

Once this is set up, invoke the CLI:

```
ifblex dl
```

# Credits

- [ibflex](https://github.com/csingley/ibflex) Python library

# License

See [LICENSE](LICENSE) file.

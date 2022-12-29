# interactive-brokers-flex-rs
Tools to assist with IB Flex reports and Ledger

The crate is incomplete but there are funcioning parts. These can be seen in Tests.

This crate contains a CLI and a library which assist with Flex Reports from Interactive Brokers.
There are several components involved:

- ✅ The command-line application (CLI) `ibflex`, provides all the features of the `ibflex` library
- ✅ `ibflex` library exposes the following functionality
  - ✅ downloans the Flex report
  - ✅ parses IB Flex report
  - [ ] runs Ledger command to retrieve the distribution transactions in the last 60 days (incomplete)
  - [ ] runs the comparison for the downloaded Cash Transactions Flex .xml report and the last 60 days in Ledger (incomplete)
- ✅ `pricedb` crate ([repo](https://github.com/alensiljak/pricedb-rust)) provides the Symbol mapping between IB Flex report and Ledger. I.e. symbol `VHYL` in the report is `VHYL_AS` in Ledger. The package is a dependency of `ibflex` but must be configured manually to read from own database.

# Usage

Downloading the Flex Query report requires Query Id and the Token. These can be passed in several ways:

1) as parameters to the download (Dl) method. Use parameters `--queryid` and `--token`.
2) as environment variables:

  The application will read the values from environment variables:

  - `IBFLEX_TOKEN`
  - `IBFLEX_QUERYID`

3) in the configuration file: The application will read the configuration file `ibflex.toml`, which is located in the current directory.

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

This will save the report in the current directory. The filename will contain today's date.

# Credits

- [ibflex](https://github.com/csingley/ibflex) Python library

# License

See [LICENSE](LICENSE) file.

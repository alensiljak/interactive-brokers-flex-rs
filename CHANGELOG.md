# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5] - 2023-01-17

### Changed

- The member names in FlexStatementResponse are now Rust's standard snake_case.

### Security

- updated dependencies

## [0.4.4] - 2023-01-11

### Fixed

- skipping transaction other than dividends and withholding tax

### Changed

- exiting comparison if no new IB transactions found

## [0.4.3] - 2023-01-11

### Fixed

- Skipping transaction other than dividends and withholding tax.

### Security

- updated dependencies

## [0.4.2] - 2023-01-05

### Added

- Expanded instructions in the ReadMe file. Added the section for the comparison command.

### Fixed

- Ignoring all but distributions and tax. Using the new enum for type comparison.

### Security

- Updated tokio dependency.

## [0.4.1] - 2023-01-05

### Added

- Display the path to the IB Flex report .xml file used for comparison.

### Fixed

- Comparing Ledger tx date to Report Date in Flex Query.

## [0.4.0] - 2023-01-04

The first acceptable version.

### Changed

- Comparison tests pass

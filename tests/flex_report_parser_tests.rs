/*
 * Test parsing the Flex report
 */

use ibflex::{
    compare::CompareParams,
    flex_query::{CashTransaction, FlexQueryResponse},
};

#[rstest::rstest]
#[test_log::test]
fn parse_file_test() {
    let mut expected = FlexQueryResponse::default();
    let tx1 = CashTransaction {
        report_date: "2022-12-14".to_string(),
        amount: "-0.91".to_string(),
        currency: "EUR".to_string(),
        date_time: "2022-12-15;12:20:00".to_string(),
        description: "TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX".to_string(),
        listing_exchange: "AEB".to_string(),
        symbol: "TCBT".to_string(),
        r#type: "Withholding Tax".to_string(),
    };
    expected
        .flex_statements
        .flex_statement
        .cash_transactions
        .cash_transaction
        .push(tx1);

    let cmp_params = CompareParams {
        flex_report_path: Some("tests/report_1.xml".to_string()),
        flex_reports_dir: None,
        ledger_journal_file: None,
        symbols_path: "tests/symbols.csv".to_owned(),
        effective_dates: false,
    };

    let report = ibflex::flex_reader::load_report(cmp_params.flex_report_path, cmp_params.flex_reports_dir);
    let actual = FlexQueryResponse::from(report);

    //assert_eq!(expected, actual);
    assert_eq!(
        expected
            .flex_statements
            .flex_statement
            .cash_transactions
            .cash_transaction[0],
        actual
            .flex_statements
            .flex_statement
            .cash_transactions
            .cash_transaction[0]
    );
}

#[test]
fn parse_string_test() {
    let content = r#"
    <FlexQueryResponse queryName="cash tx, last 30 days" type="AF">
    <FlexStatements count="1">
        <FlexStatement accountId="U3550519" fromDate="2022-11-24" toDate="2022-12-23" period="Last30CalendarDays" whenGenerated="2022-12-25;14:53:12">
            <CashTransactions>
                <CashTransaction reportDate="2022-12-14" dateTime="2022-12-15;12:20:00" symbol="TCBT" listingExchange="AEB" type="Withholding Tax" amount="-0.91" currency="EUR" description="TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX" />
                <CashTransaction reportDate="2022-12-15" dateTime="2022-12-15;12:20:00" symbol="TRET" listingExchange="AEB" type="Withholding Tax" amount="-5.77" currency="EUR" description="TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE - NL TAX" />
                <CashTransaction reportDate="2022-12-14" dateTime="2022-12-15;12:20:00" symbol="TCBT" listingExchange="AEB" type="Dividends" amount="6.05" currency="EUR" description="TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE (Ordinary Dividend)" />
                <CashTransaction reportDate="2022-12-15" dateTime="2022-12-15;12:20:00" symbol="TRET" listingExchange="AEB" type="Dividends" amount="38.4" currency="EUR" description="TRET(NL0009690239) CASH DIVIDEND EUR 0.30 PER SHARE (Ordinary Dividend)" />
                <CashTransaction reportDate="2022-11-30" dateTime="2022-11-30;16:00:00" symbol="" listingExchange="" type="Deposits/Withdrawals" amount="1500" currency="EUR" description="CASH RECEIPTS / ELECTRONIC FUND TRANSFERS" />
                <CashTransaction reportDate="2022-12-05" dateTime="2022-12-05;16:00:00" symbol="" listingExchange="" type="Broker Interest Received" amount="2.77" currency="AUD" description="AUD CREDIT INT FOR NOV-2022" />
                <CashTransaction reportDate="2022-11-25" dateTime="2022-11-25" symbol="DGS" listingExchange="ARCA" type="Commission Adjustments" amount="0.33225725" currency="USD" description="Refund (DGS, 10, 2022-10-26)" />
            </CashTransactions>
        </FlexStatement>
    </FlexStatements>
</FlexQueryResponse>
    "#.to_string();
    let actual = FlexQueryResponse::from(content);

    assert_ne!(actual, FlexQueryResponse::default());
    assert_eq!(actual.flex_statements.count, 1);
    // statement
    let stmt = &actual.flex_statements.flex_statement;
    assert_eq!("U3550519".to_string(), stmt.account_id);
    assert_eq!("2022-11-24", stmt.from_date);
    assert_eq!("2022-12-23", stmt.to_date);
    assert_eq!("Last30CalendarDays", stmt.period);
    assert_eq!("2022-12-25;14:53:12", stmt.when_generated);
    // cash transactions
    assert_eq!(
        7,
        actual
            .flex_statements
            .flex_statement
            .cash_transactions
            .cash_transaction
            .len()
    );
    // cash transaction
    let tx1 = &actual
        .flex_statements
        .flex_statement
        .cash_transactions
        .cash_transaction[0];
    assert_eq!("2022-12-14", tx1.report_date);
    assert_eq!("2022-12-15;12:20:00", tx1.date_time);
    assert_eq!("TCBT", tx1.symbol);
    assert_eq!("AEB", tx1.listing_exchange);
    assert_eq!("Withholding Tax", tx1.r#type);
    assert_eq!("-0.91", tx1.amount);
    assert_eq!("EUR", tx1.currency);
    assert_eq!(
        "TCBT(NL0009690247) CASH DIVIDEND EUR 0.05 PER SHARE - NL TAX",
        tx1.description
    );
}

/*!
 * Flex Query enums
 * 
 * Original:
 * https://github.com/alensiljak/ibflex/blob/master/ibflex/enums.py
 */

pub fn cash_action(action: &str) -> &str {
    match action {
        "Deposits & Withdrawals" => "DEPOSITWITHDRAW",
        "Broker Interest Paid" => todo!(),
        "Broker Interest Received" => todo!(),
        "Withholding Tax" => "WHTAX",
        "Bond Interest Received" => todo!(),
        "Bond Interest Paid" => todo!(),
        "Other Fees" => todo!(),
        "Dividends" => "DIVIDEND",
        "Payment In Lieu Of Dividends" => todo!(),
        "Commission Adjustments" => todo!(),
        _ => panic!("Unrecognized cash action type!")
    }
}

#[cfg(test)]
mod tests {
    use super::cash_action;

    #[test]
    fn test_mapping() {
        let ib_type = "Withholding Tax";
        let actual = cash_action(ib_type);

        assert_eq!("WHTAX", actual);
    }
}
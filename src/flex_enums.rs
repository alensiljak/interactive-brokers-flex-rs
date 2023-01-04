/*!
 * Flex Query enums
 * 
 * Original:
 * https://github.com/alensiljak/ibflex/blob/master/ibflex/enums.py
 */

use std::fmt::Display;

#[derive(Debug)]
pub enum CashAction {
    DEPOSITWITHDRAW,
    BROKERINTPAID,
    BROKERINTRCVD,
    WHTAX,
    BONDINTRCVD,
    BONDINTPAID,
    FEES,
    DIVIDEND,
    PAYMENTINLIEU,
    COMMADJ
}

impl Display for CashAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn cash_action(action: &str) -> String {
    match action {
        // "Deposits & Withdrawals" => "DEPOSITWITHDRAW",
        "Deposits/Withdrawals" => CashAction::DEPOSITWITHDRAW.to_string(),
        "Broker Interest Paid" => CashAction::BROKERINTPAID.to_string(),
        "Broker Interest Received" => CashAction::BROKERINTRCVD.to_string(),
        "Withholding Tax" => CashAction::WHTAX.to_string(),
        "Bond Interest Received" => CashAction::BONDINTRCVD.to_string(),
        "Bond Interest Paid" => CashAction::BONDINTPAID.to_string(),
        "Other Fees" => CashAction::FEES.to_string(),
        "Dividends" => CashAction::DIVIDEND.to_string(),
        "Payment In Lieu Of Dividends" => CashAction::PAYMENTINLIEU.to_string(),
        "Commission Adjustments" => CashAction::COMMADJ.to_string(),
        _ => panic!("Unrecognized cash action type: {}", action)
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
/*!
 * Flex Query enums
 * 
 * Original:
 * https://github.com/alensiljak/ibflex/blob/master/ibflex/enums.py
 */

use std::fmt::Display;

#[derive(Debug)]
pub enum CashAction {
    DepositWithdraw,
    BrokerIntPaid,
    BrokerIntRcvd,
    WhTax,
    BondIntRcvd,
    BondIntPaid,
    Fees,
    Dividend,
    PaymentInLieu,
    CommAdj
}

impl Display for CashAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Translates the IB Flex cash action name into the CashAction enum variant.
/// Example:
/// let type = cash_action("Deposits/Withdrawals");
pub fn cash_action(action: &str) -> String {
    match action {
        // "Deposits & Withdrawals" => "DEPOSITWITHDRAW",
        "Deposits/Withdrawals" => CashAction::DepositWithdraw.to_string(),
        "Broker Interest Paid" => CashAction::BrokerIntPaid.to_string(),
        "Broker Interest Received" => CashAction::BrokerIntRcvd.to_string(),
        "Withholding Tax" => CashAction::WhTax.to_string(),
        "Bond Interest Received" => CashAction::BondIntRcvd.to_string(),
        "Bond Interest Paid" => CashAction::BondIntPaid.to_string(),
        "Other Fees" => CashAction::Fees.to_string(),
        "Dividends" => CashAction::Dividend.to_string(),
        "Payment In Lieu Of Dividends" => CashAction::PaymentInLieu.to_string(),
        "Commission Adjustments" => CashAction::CommAdj.to_string(),
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

        assert_eq!("WhTax", actual);
    }
}
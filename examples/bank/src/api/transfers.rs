#![allow(unused_variables)]

use domain::Money;
use rocket_contrib::JSON;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferMoneyData {
    pub debit_account: String,
    pub credit_account: String,
    pub amount: Money,
}

#[post("/transfers", data = "<data>")]
pub fn transfer_money(data: JSON<TransferMoneyData>) {
    unimplemented!()
}

#![allow(unused_variables)]

use domain::Money;
use rocket_contrib::JSON;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferMoneyData {
    pub debit_account: Uuid,
    pub credit_account: Uuid,
    pub amount: Money,
}

#[post("/transfers", data = "<data>")]
pub fn transfer_money(data: JSON<TransferMoneyData>) {
    unimplemented!()
}

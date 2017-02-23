#![allow(unused_variables)]

use domain::Money;
use rocket_contrib::JSON;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct OpenAccountData {
    pub initial_balance: Money,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DepositMoneyData {
    pub transfer_id: String,
    pub amount: Money,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawMoneyData {
    pub transfer_id: String,
    pub amount: Money,
}

#[post("/accounts", data = "<data>")]
pub fn open_account(data: JSON<OpenAccountData>) {
    unimplemented!()
}

#[post("/accounts/<id>/deposit_money", data = "<data>")]
pub fn deposit_money(id: &str, data: JSON<DepositMoneyData>) {
    unimplemented!()
}

#[post("/accounts/<id>/withdraw_money", data = "<data>")]
pub fn withdraw_money(id: &str, data: JSON<WithdrawMoneyData>) {
    unimplemented!()
}

#[post("/accounts/<id>/close_account")]
pub fn close_account(id: &str) {
    unimplemented!()
}

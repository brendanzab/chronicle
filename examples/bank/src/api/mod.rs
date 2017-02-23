use rocket;

pub mod accounts;
pub mod transfers;

pub fn launch() {
    rocket::ignite()
        .mount("/api/",
               routes![
            accounts::open_account,
            accounts::deposit_money,
            accounts::withdraw_money,
            accounts::close_account,
            transfers::transfer_money,
        ])
        .launch();
}

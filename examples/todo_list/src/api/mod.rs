use rocket;

pub mod tasks;

pub fn launch() {
    rocket::ignite()
        .mount("/api/",
               routes![
            tasks::open_account,
            tasks::deposit_money,
            tasks::complete,
            tasks::archive,
        ])
        .launch();
}

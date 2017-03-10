use chronicle_memory::MemoryEventStore;
use rocket;

use domain::task::Event;

pub mod tasks;

pub fn launch(event_store: MemoryEventStore<Event>) {
    rocket::ignite()
        .mount("/api/",
               routes![
            tasks::create,
            tasks::change_description,
            tasks::complete,
            tasks::archive,
        ])
        .manage(event_store)
        .launch();
}

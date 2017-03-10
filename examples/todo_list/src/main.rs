extern crate chronicle_memory;
extern crate todo_list_example;
extern crate rocket;
extern crate futures;


use chronicle_memory::MemoryEventStore;


fn main() {
    let event_store = MemoryEventStore::new();

    todo_list_example::api::launch(event_store);
}

#![allow(unused_variables)]


use chronicle::EventStore;
use chronicle_domain::Aggregate;
use chronicle_memory::{MemoryEventStore, EventsStreamError};
use futures::{Future, Stream};
use rocket::State;
use rocket_contrib::{JSON, UUID, Value};
use uuid::Uuid;

use domain::task::{Command, Event, Task, State as TaskState};


#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskData {
    pub description: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct ChangeDescriptionData {
    pub description: String,
}


fn initial_state<E>(id: Uuid,
                    event_store: &MemoryEventStore<Event>)
                    -> impl Future<Item = Option<TaskState>, Error = E> {
    fn cast_error<T>(err: EventsStreamError) -> T {
        match err {}
    }

    event_store.events(id, 0)
        .map_err(cast_error::<E>)
        .fold(Task::initial_state(), |mut state, persisted_event| {
            Task::apply_event(&mut state, persisted_event.event);
            Ok(state)
        })
}


#[post("/tasks", format = "application/json", data = "<data>")]
pub fn create(data: JSON<CreateTaskData>,
              event_store: State<MemoryEventStore<Event>>)
              -> JSON<Value> {
    let id = Uuid::new_v4();
    let data = data.into_inner();
    let command = Command::Create(data.description);

    initial_state(id, &event_store)
        .and_then(|state| Task::handle_command(&state, command))
        .and_then(|events| Ok(event_store.append_events(id, events)))
        .wait()
        .unwrap();

    JSON(json!({
        "id": id,
    }))
}


#[post("/tasks/<id>/change_description", format = "application/json", data = "<data>")]
pub fn change_description(id: UUID,
                          data: JSON<ChangeDescriptionData>,
                          event_store: State<MemoryEventStore<Event>>) {
    let id = id.into_inner();
    let data = data.into_inner();
    let command = Command::ChangeDescription(data.description);

    initial_state(id, &event_store)
        .and_then(|state| Task::handle_command(&state, command))
        .and_then(|events| Ok(event_store.append_events(id, events)))
        .wait()
        .unwrap();
}


#[post("/tasks/<id>/complete", format = "application/json")]
pub fn complete(id: UUID, event_store: State<MemoryEventStore<Event>>) {
    let id = id.into_inner();
    let command = Command::Complete;

    initial_state(id, &event_store)
        .and_then(|state| Task::handle_command(&state, command))
        .and_then(|events| Ok(event_store.append_events(id, events)))
        .wait()
        .unwrap();
}


#[post("/tasks/<id>/archive", format = "application/json")]
pub fn archive(id: UUID, event_store: State<MemoryEventStore<Event>>) {
    let id = id.into_inner();
    let command = Command::Archive;

    initial_state(id, &event_store)
        .and_then(|state| Task::handle_command(&state, command))
        .and_then(|events| Ok(event_store.append_events(id, events)))
        .wait()
        .unwrap();
}

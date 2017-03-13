extern crate chronicle;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate futures;
extern crate uuid;


use chronicle::{EventStore, PersistedEvent};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use futures::{Async, Poll, Stream};
use uuid::Uuid;


mod models;
mod schema;


embed_migrations!("migrations");


struct EventStoreConnection {
    connection: PgConnection,
}


impl EventStore for EventStoreConnection {
    type Offset = i64;
    type Event = Vec<u8>;
    type EventsStream = EventsStream;

    fn append_events(&self, source_id: Uuid, events: Vec<Vec<u8>>) {
        unimplemented!()
    }

    fn events(&self, source_id: Uuid, offset: i64) -> EventsStream {
        unimplemented!()
    }
}


pub struct EventsStream {
    source_id: Uuid,
    sequence_number: i64,
    offset: i64,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventsStreamError {}


impl Stream for EventsStream {
    type Item = PersistedEvent<i64, Vec<u8>>;
    type Error = EventsStreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, EventsStreamError> {
        unimplemented!()
    }
}

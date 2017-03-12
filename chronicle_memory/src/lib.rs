//! An in-memory event store, useful for testing
//!
//! # Example
//!
//! ```rust
//! extern crate chronicle;
//! extern crate chronicle_memory;
//! extern crate futures;
//! #[macro_use]
//! extern crate lazy_static;
//! extern crate uuid;
//!
//!
//! use chronicle_memory::MemoryEventStore;
//!
//!
//! lazy_static! {
//!     /// A globally accessible event store
//!     static ref EVENT_STORE: MemoryEventStore<&'static str> = {
//!         MemoryEventStore::new()
//!     };
//! }
//!
//! fn main() {
//!     use chronicle::EventStore;
//!     use futures::{Future, Stream, future};
//!     use std::thread;
//!     use uuid::Uuid;
//!
//!
//!     // Some unique source ids
//!     let id_1 = Uuid::new_v4();
//!     let id_2 = Uuid::new_v4();
//!
//!     // Some sample source id and payload pairs
//!     let events = vec![
//!         (id_1, vec!["1", "2", "3"]),
//!         (id_1, vec!["4", "5"]),
//!         (id_2, vec!["A", "B"]),
//!         (id_2, vec!["C"]),
//!         (id_2, vec!["D", "E", "F"]),
//!     ];
//!
//!     // The event payloads partitioned by source id and sorted by value
//!     let events_1 = vec!["1", "2", "3", "4", "5"];
//!     let events_2 = vec!["A", "B", "C", "D", "E", "F"];
//!
//!
//!     // Append all the events - let's not worry about ordering
//!     let handles = events.into_iter().map(|(source_id, events)| {
//!         thread::spawn(move || EVENT_STORE.append_events(source_id, events))
//!     });
//!
//!     for handle in handles.collect::<Vec<_>>() {
//!         handle.join().unwrap();
//!     }
//!
//!
//!     // We'll expect to get the same number of events that we gave for each id
//!     let events_stream_1 = EVENT_STORE.events(id_1, 0);
//!     let events_stream_2 = EVENT_STORE.events(id_2, 0);
//!
//!     let (mut collected_events_1, mut collected_events_2) =
//!         Future::join(events_stream_1.map(|e| e.payload).collect(),
//!                      events_stream_2.map(|e| e.payload).collect()).wait().unwrap();
//!
//!     // We don't know the order that the events came in, so we need to
//!     // ensure that they are all sorted first
//!     collected_events_1.sort();
//!     collected_events_2.sort();
//!
//!     // Check that we have the expected items!
//!     assert_eq!(collected_events_1, events_1);
//!     assert_eq!(collected_events_2, events_2);
//! }
//! ```


extern crate chashmap;
extern crate chronicle;
extern crate futures;
extern crate uuid;


use chashmap::CHashMap;
use chronicle::{EventStore, PersistedEvent};
use futures::{Async, Poll, Stream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;


/// An in-memory event store implementation that can be concurrently accessed
#[derive(Debug, Clone)]
pub struct MemoryEventStore<Event> {
    offset: Arc<AtomicUsize>,
    /// The stored event payloads and their global offset number
    events: CHashMap<Uuid, Vec<(usize, Event)>>,
}


impl<Event> MemoryEventStore<Event> {
    /// Create an empty event store
    pub fn new() -> MemoryEventStore<Event> {
        MemoryEventStore {
            offset: Arc::new(AtomicUsize::new(0)),
            events: CHashMap::new(),
        }
    }
}



impl<Event> EventStore for MemoryEventStore<Event>
    where Event: Clone
{
    type Offset = usize;
    type Event = Event;
    type EventsStream = EventsStream<Event>;

    fn append_events(&self, source_id: Uuid, events: Vec<Event>) {
        if events.is_empty() {
            return;
        }

        self.events.alter(source_id, |existing_events| {
            let mut existing_events = existing_events.unwrap_or(Vec::new());
            let new_events = events.into_iter().map(|event| {
                // Keep the global offset up to date as we iterate. Opting
                // for the strongest, sequentially consistent memory ordering
                // for now. We may be able to relax this though... ¯\_(ツ)_/¯
                let offset = self.offset.fetch_add(1, Ordering::SeqCst);
                (offset, event)
            });

            existing_events.extend(new_events);

            Some(existing_events)
        });
    }

    fn events(&self, source_id: Uuid, offset: Self::Offset) -> EventsStream<Event> {
        EventsStream {
            source_id: source_id,
            sequence_number: 0,
            offset: offset,
            event_store: self.clone(),
        }
    }
}


/// A stream of events for a specified source id
pub struct EventsStream<Event> {
    source_id: Uuid,
    sequence_number: usize,
    offset: usize,
    event_store: MemoryEventStore<Event>,
}


/// An error that may be returned when polling on the `EventsStream`
///
/// Note that this error has no variants, so can never happen. This will be
/// replaced by `!` once `#![feature(never_type)]` has been stabilised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventsStreamError {}


impl<Event> Stream for EventsStream<Event>
    where Event: Clone
{
    type Item = PersistedEvent<usize, Event>;
    type Error = EventsStreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, EventsStreamError> {
        use chronicle::SequenceNumber;

        if let Some(source_events) = self.event_store.events.get(&self.source_id) {
            while let Some(&(offset, ref payload)) = source_events.get(self.sequence_number) {
                let sequence_number = self.sequence_number;
                self.sequence_number += 1;

                if offset < self.offset {
                    continue;
                } else {
                    let persisted_event = PersistedEvent {
                        source_id: self.source_id,
                        offset: offset,
                        sequence_number: sequence_number as SequenceNumber,
                        payload: payload.clone(),
                    };

                    return Ok(Async::Ready(Some(persisted_event)));
                }
            }
        }

        Ok(Async::Ready(None))
    }
}


#[cfg(test)]
mod tests {
    use chronicle::{EventStore, PersistedEvent};
    use futures::Future;
    use uuid::Uuid;

    use super::*;


    #[test]
    fn append_events_if_none_exist_for_the_source_id() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B", "C"]);

        assert_eq!(event_store.events.get(&source_id_1).map(|es| es.clone()),
                   Some(vec![(0, "A"), (1, "B"), (2, "C")]));
    }


    #[test]
    fn append_events_for_an_existing_source_id() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B", "C"]);
        event_store.append_events(source_id_1, vec![]);
        event_store.append_events(source_id_1, vec!["D", "E"]);

        assert_eq!(event_store.events.get(&source_id_1).map(|es| es.clone()),
                   Some(vec![(0, "A"), (1, "B"), (2, "C"), (3, "D"), (4, "E")]));
    }


    #[test]
    fn append_events_maintianing_the_offsets() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();
        let source_id_2 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B", "C"]);
        event_store.append_events(source_id_2, vec!["a", "b"]);
        event_store.append_events(source_id_1, vec!["D", "E"]);
        event_store.append_events(source_id_2, vec!["c", "d"]);

        assert_eq!(event_store.events.get(&source_id_1).map(|es| es.clone()),
                   Some(vec![(0, "A"), (1, "B"), (2, "C"), (5, "D"), (6, "E")]));
        assert_eq!(event_store.events.get(&source_id_2).map(|es| es.clone()),
                   Some(vec![(3, "a"), (4, "b"), (7, "c"), (8, "d")]));
    }

    #[test]
    fn events_on_empty_store() {
        let event_store = MemoryEventStore::<()>::new();
        let source_id_1 = Uuid::new_v4();

        let events = event_store.events(source_id_1, 0).collect().wait();

        assert_eq!(events, Ok(Vec::new()));
    }

    #[test]
    fn events_on_non_empty_store() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B", "C"]);

        let events = event_store.events(source_id_1, 0).collect().wait();

        assert_eq!(events,
                   Ok(vec![PersistedEvent {
                               offset: 0,
                               source_id: source_id_1,
                               sequence_number: 0,
                               payload: "A",
                           },
                           PersistedEvent {
                               offset: 1,
                               source_id: source_id_1,
                               sequence_number: 1,
                               payload: "B",
                           },
                           PersistedEvent {
                               offset: 2,
                               source_id: source_id_1,
                               sequence_number: 2,
                               payload: "C",
                           }]));
    }

    #[test]
    fn events_with_out_of_range_offset() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B", "C"]);

        let events = event_store.events(source_id_1, 100).collect().wait();

        assert_eq!(events, Ok(Vec::new()));
    }

    #[test]
    fn events_with_non_contiguous_global_id_sequence() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();
        let source_id_2 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A"]);
        event_store.append_events(source_id_2, vec!["1", "2"]);
        event_store.append_events(source_id_1, vec!["B", "C"]);

        assert_eq!(event_store.events(source_id_1, 0).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 0,
                               source_id: source_id_1,
                               sequence_number: 0,
                               payload: "A",
                           },
                           PersistedEvent {
                               offset: 3,
                               source_id: source_id_1,
                               sequence_number: 1,
                               payload: "B",
                           },
                           PersistedEvent {
                               offset: 4,
                               source_id: source_id_1,
                               sequence_number: 2,
                               payload: "C",
                           }]));

        assert_eq!(event_store.events(source_id_2, 0).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 1,
                               source_id: source_id_2,
                               sequence_number: 0,
                               payload: "1",
                           },
                           PersistedEvent {
                               offset: 2,
                               source_id: source_id_2,
                               sequence_number: 1,
                               payload: "2",
                           }]));
    }

    #[test]
    fn events_after_offset_id() {
        let event_store = MemoryEventStore::new();
        let source_id_1 = Uuid::new_v4();
        let source_id_2 = Uuid::new_v4();

        event_store.append_events(source_id_1, vec!["A", "B"]);
        event_store.append_events(source_id_2, vec!["1", "2", "3"]);
        event_store.append_events(source_id_1, vec!["C", "D"]);

        assert_eq!(event_store.events(source_id_1, 1).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 1,
                               source_id: source_id_1,
                               sequence_number: 1,
                               payload: "B",
                           },
                           PersistedEvent {
                               offset: 5,
                               source_id: source_id_1,
                               sequence_number: 2,
                               payload: "C",
                           },
                           PersistedEvent {
                               offset: 6,
                               source_id: source_id_1,
                               sequence_number: 3,
                               payload: "D",
                           }]));

        assert_eq!(event_store.events(source_id_1, 2).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 5,
                               source_id: source_id_1,
                               sequence_number: 2,
                               payload: "C",
                           },
                           PersistedEvent {
                               offset: 6,
                               source_id: source_id_1,
                               sequence_number: 3,
                               payload: "D",
                           }]));

        assert_eq!(event_store.events(source_id_1, 5).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 5,
                               source_id: source_id_1,
                               sequence_number: 2,
                               payload: "C",
                           },
                           PersistedEvent {
                               offset: 6,
                               source_id: source_id_1,
                               sequence_number: 3,
                               payload: "D",
                           }]));

        assert_eq!(event_store.events(source_id_1, 6).collect().wait(),
                   Ok(vec![PersistedEvent {
                               offset: 6,
                               source_id: source_id_1,
                               sequence_number: 3,
                               payload: "D",
                           }]));
    }
}

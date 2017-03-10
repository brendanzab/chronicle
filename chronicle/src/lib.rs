extern crate futures;
extern crate uuid;


use futures::Stream;
use uuid::Uuid;


/// An event with associated metadata that corresponds to how it was stored in
/// the event store.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistedEvent<Offset, Event> {
    /// The unique identifier of a single source of events in the event store.
    /// This usually corresponds to an 'aggregate id' in DDD.
    pub source_id: Uuid,
    /// The global offset of the event in the event store
    pub global_offset: Offset,
    /// The offset within a single source
    pub source_offset: Offset,
    /// The event that was stored by the client of the event store
    pub event: Event,
}


impl<Offset, Event> PersistedEvent<Offset, Event> {
    /// Take the event data by reference, cloning the source id and offsets
    pub fn as_ref(&self) -> PersistedEvent<Offset, &Event>
        where Offset: Clone,
    {
        PersistedEvent {
            source_id: self.source_id,
            global_offset: self.global_offset.clone(),
            source_offset: self.source_offset.clone(),
            event: &self.event,
        }
    }

    /// Apply a transformation to the event
    pub fn map<NewEvent, F>(self, f: F) -> PersistedEvent<Offset, NewEvent>
        where F: FnOnce(Event) -> NewEvent,
    {
        PersistedEvent {
            source_id: self.source_id,
            global_offset: self.global_offset,
            source_offset: self.source_offset,
            event: f(self.event),
        }
    }
}


/// A repository for storing historical event logs.
pub trait EventStore {
    /// The type of the offests into the event store and its sources.
    /// Relational DBs will probably use a monotonically increasing sequence
    /// number, where as distributed data sources may use timestamps.
    type Offset;

    /// The event store's native serialized form
    type Event;

    /// A lazily loaded source of events
    type EventsStream: Stream<Item = PersistedEvent<Self::Offset, Self::Event>>;

    /// Append the events to the event store for the specified source id
    fn append_events(&self, source_id: Uuid, events: Vec<Self::Event>);

    /// Stream the events back from the event store for the specified source id
    fn events(&self, source_id: Uuid, global_offset: Self::Offset) -> Self::EventsStream;
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;


    #[test]
    fn persisted_event_map() {
        let event = PersistedEvent {
            source_id: Uuid::new_v4(),
            global_offset: 123,
            source_offset: 354,
            event: "hello",
        };

        let new_event = event.clone().map(String::from);

        assert_eq!(new_event.source_id, event.source_id);
        assert_eq!(new_event.global_offset, event.global_offset);
        assert_eq!(new_event.source_offset, event.source_offset);
        assert_eq!(new_event.event, event.event);
    }


    #[test]
    fn persisted_event_as_ref() {
        let event = PersistedEvent {
            source_id: Uuid::new_v4(),
            global_offset: 123,
            source_offset: 354,
            event: "hello",
        };

        let new_event = event.as_ref();

        assert_eq!(new_event.source_id, event.source_id);
        assert_eq!(new_event.global_offset, event.global_offset);
        assert_eq!(new_event.source_offset, event.source_offset);
        assert_eq!(*new_event.event, event.event);
    }
}

extern crate futures;
extern crate uuid;


use futures::Stream;
use uuid::Uuid;


/// The sequence number within a single source of events
pub type SequenceNumber = u32;


/// An event with associated metadata that corresponds to how it was stored in
/// the event store.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistedEvent<Offset, Event> {
    /// The global offset of the event in the event store
    pub offset: Offset,
    /// The unique identifier of a single source of events in the event store.
    /// This usually corresponds to an 'aggregate id' in DDD.
    pub source_id: Uuid,
    /// The sequence number within a single source of events
    pub sequence_number: SequenceNumber,
    /// The event payload that was stored by the client of the event store
    pub payload: Event,
}


impl<Offset, Event> PersistedEvent<Offset, Event> {
    /// Take the event data by reference, copying the other fields
    pub fn as_ref(&self) -> PersistedEvent<Offset, &Event>
        where Offset: Clone
    {
        PersistedEvent {
            offset: self.offset.clone(),
            source_id: self.source_id,
            sequence_number: self.sequence_number.clone(),
            payload: &self.payload,
        }
    }

    /// Apply a transformation to the event payload, copying the other fields
    pub fn map<NewEvent, F>(self, f: F) -> PersistedEvent<Offset, NewEvent>
        where F: FnOnce(Event) -> NewEvent
    {
        PersistedEvent {
            offset: self.offset,
            source_id: self.source_id,
            sequence_number: self.sequence_number,
            payload: f(self.payload),
        }
    }
}


/// A repository for storing historical event logs.
pub trait EventStore {
    /// The type of the offests into the event store. Relational DBs
    /// will probably use a monotonically increasing sequence number,
    /// where as distributed data sources may use timestamps.
    type Offset: PartialOrd;

    /// The event store's native serialized form
    type Event;

    /// A lazily loaded source of events
    type EventsStream: Stream<Item = PersistedEvent<Self::Offset, Self::Event>>;

    /// Append the events to the event store for the specified source id
    fn append_events(&self, source_id: Uuid, events: Vec<Self::Event>);

    /// Stream the events back from the event store for the specified source id
    fn events(&self, source_id: Uuid, offset: Self::Offset) -> Self::EventsStream;
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;


    #[test]
    fn persisted_event_map() {
        let event = PersistedEvent {
            offset: 123,
            source_id: Uuid::new_v4(),
            sequence_number: 354,
            payload: "hello",
        };

        let new_event = event.clone().map(String::from);

        assert_eq!(new_event.offset, event.offset);
        assert_eq!(new_event.source_id, event.source_id);
        assert_eq!(new_event.sequence_number, event.sequence_number);
        assert_eq!(new_event.payload, event.payload);
    }


    #[test]
    fn persisted_event_as_ref() {
        let event = PersistedEvent {
            offset: 123,
            source_id: Uuid::new_v4(),
            sequence_number: 354,
            payload: "hello",
        };

        let new_event = event.as_ref();

        assert_eq!(new_event.offset, event.offset);
        assert_eq!(new_event.source_id, event.source_id);
        assert_eq!(new_event.sequence_number, event.sequence_number);
        assert_eq!(new_event.payload, &event.payload);
    }
}

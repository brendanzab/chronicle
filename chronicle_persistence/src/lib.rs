extern crate futures;


use futures::Stream;


/// An event with associated metadata that corresponds to how it was stored in
/// the event store.
pub struct PersistedEvent<Offset, Event> {
    /// The global offset of the event in the event store.
    pub global_offset: Offset,
    /// The offset within a single stream.
    pub stream_offset: Offset,
    /// The event that was stored by the client of the event store.
    pub event: Event,
}


/// A repository for storing historical event logs.
pub trait EventStore {
    /// The unique identifier of a stream in the event store
    type StreamId;

    /// The type of the offests into the event store and its streams
    type Offset;

    /// The event store's native serialized form
    type SerializedEvent;

    /// A lazily loaded stream of events in the event store's native form
    type EventStream: Stream<Item = PersistedEvent<Self::Offset, Self::SerializedEvent>>;

    /// Persist the events to the event store
    fn persist_events<Events>(&self,
                              stream_id: &Self::StreamId,
                              stream_offset: Self::Offset,
                              events: Events)
        where Events: IntoIterator<Item = Self::SerializedEvent>;

    /// Read the events back from the event store
    fn get_events(&self, stream_id: &Self::StreamId) -> Self::EventStream;
}

pub trait SnapshotStore {}

pub trait ProjectionStore {}

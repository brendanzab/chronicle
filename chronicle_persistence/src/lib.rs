pub trait EventStore {
    type Id;
    type Event;

    fn write_events<Events>(&self, aggregate_id: &Self::Id, events: Events)
        where Events: IntoIterator<Item = Self::Event>;

    fn read_events(&self, aggregate_id: &Self::Id) -> Vec<Self::Event>;
}

pub trait SnapshotStore {}

pub trait ProjectionStore {}

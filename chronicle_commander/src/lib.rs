extern crate chronicle_persistence;
extern crate futures;

use futures::Future;

pub trait Aggregate {
    type State;
    type Event;
    type Command;
    type ValidationError;
    type EventsFuture: Future<Item = Vec<Self::Event>, Error = Self::ValidationError>;

    fn initial_state() -> Self::State;
    fn handle_command(state: &Self::State, command: Self::Command) -> Self::EventsFuture;
    fn apply_event(state: &mut Self::State, event: Self::Event);
}

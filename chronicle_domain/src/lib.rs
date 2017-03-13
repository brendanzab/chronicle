#![feature(conservative_impl_trait)]
#![feature(never_type)]


extern crate futures;
extern crate uuid;


use futures::IntoFuture;


pub mod commands;


/// An aggregate that is responsible for validating and applying
/// commands.
///
/// Aggregates may consist of one domain object, or be composed of a cluster
/// of domain objects. They should in general be designed in such that
/// transactions do not cross aggregate boundaries. Doing so increases the
/// complexity of maintaining a valid state, and requires the use of something
/// like sagas (not yet implemented).
///
/// # References
///
/// - https://martinfowler.com/bliki/DDD_Aggregate.html
pub trait Aggregate {
    /// A snapshot of the state of an aggregate. More often than not this is an
    /// `Option<T>`, to signify whether the aggregate has been instantiated or
    /// not.
    type State;

    /// The result of a successful command.
    type Event;

    /// A command to update the state of the aggregate. This may or may not
    /// result in an `Event` being comitted.
    type Command;

    /// An error that may be yeilded by the `handle_command` function on a
    /// validation error.
    type CommandError;

    /// A future that will be returned by the `handle_command` function.
    type EventsFuture: IntoFuture<Item = Vec<Self::Event>, Error = Self::CommandError>;

    /// The seed state, before the aggregate has been created.
    fn initial_state() -> Self::State;

    /// Handle a command based on the current state, returning a future that
    /// either yeilds a vector of resulting `Event`s or a `CommandError`.
    fn handle_command(state: &Self::State, command: Self::Command) -> Self::EventsFuture;

    /// Apply an event to the state of an aggregate. Note that this should
    /// always succeed.
    fn apply_event(state: &mut Self::State, event: Self::Event);
}

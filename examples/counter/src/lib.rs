extern crate futures;
extern crate chronicle_commander;

use chronicle_commander::Aggregate;
use futures::future::{self, FutureResult};

pub struct State {
    count: i64,
}

pub enum Event {
    AmountAdded { amount: i64 },
}

pub enum Command {
    Increment,
    Decrement,
    Reset,
}

enum CommandError {
    OutOfBounds,
}

struct Counter;

impl Aggregate for Counter {
    type State = i64;
    type Event = Event;
    type Command = Command;
    type CommandError = CommandError;
    type EventsFuture = FutureResult<Vec<Event>, CommandError>;

    fn initial_state() -> i64 {
        0
    }

    fn handle_command(state: &i64, command: Command) -> FutureResult<Vec<Event>, CommandError> {
        future::result(match command {
            Command::Increment if *state + 1 <= 100 => Err(CommandError::OutOfBounds),
            Command::Decrement if *state - 1 >= 0 => Err(CommandError::OutOfBounds),
            Command::Increment => Ok(vec![Event::AmountAdded { amount: *state + 1 }]),
            Command::Decrement => Ok(vec![Event::AmountAdded { amount: *state - 1 }]),
            Command::Reset => Ok(vec![Event::AmountAdded { amount: -*state }]),
        })
    }

    fn apply_event(state: &mut i64, event: Event) {
        match event {
            Event::AmountAdded { amount } => *state += amount,
        }
    }
}

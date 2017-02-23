extern crate chronicle_persistence;

pub trait Aggregate {
    type State;
    type Event;
    type Command;
    type ValidationError;

    fn initial_state() -> Self::State;
    fn apply_event(state: &mut Self::State, command: Self::Event);
    fn handle_command(state: &Self::State,
                      command: Self::Command)
                      -> Result<Vec<Self::Event>, Self::ValidationError>;
}

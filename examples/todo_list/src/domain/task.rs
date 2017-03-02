use chronicle_commander::Aggregate;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Created { description: String },
    DescriptionChanged { description: String },
    Completed,
    Archived,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Create(String),
    ChangeDescription(String),
    Complete,
    Archive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    NotYetCreated,
    AlreadyCreated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Completed,
    Archived,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub description: String,
    pub status: Status,
}

impl State {
    pub fn new(description: String, status: Status) -> State {
        State {
            description: description,
            status: status,
        }
    }
}

pub struct Task;

impl Aggregate for Task {
    type State = Option<State>;
    type Event = Event;
    type Command = Command;
    type CommandError = CommandError;
    type EventsFuture = Result<Vec<Event>, CommandError>;

    fn initial_state() -> Option<State> {
        None
    }

    fn handle_command(state: &Option<State>, command: Command) -> Result<Vec<Event>, CommandError> {
        use self::Command::*;
        use self::Event::*;

        match *state {
            None => {
                match command {
                    Create(description) => Ok(vec![Created { description: description }]),
                    _ => Err(CommandError::NotYetCreated),
                }
            }
            Some(ref state) => {
                match command {
                    Create(_) => Err(CommandError::AlreadyCreated),
                    ChangeDescription(description) => {
                        if description == state.description {
                            Ok(vec![])
                        } else {
                            Ok(vec![DescriptionChanged { description: description }])
                        }
                    }
                    Complete if state.status == Status::Completed => Ok(vec![]),
                    Archive if state.status == Status::Archived => Ok(vec![]),
                    Complete => Ok(vec![Event::Completed]),
                    Archive => Ok(vec![Event::Archived]),
                }
            }
        }
    }

    fn apply_event(state: &mut Option<State>, event: Event) {
        use self::Event::*;

        match *state {
            None => {
                match event {
                    Created { description } => {
                        *state = Some(State::new(description, Status::Active))
                    }
                    _ => (), // TODO: Log?
                }
            }
            Some(ref mut state) => {
                match event {
                    Created { .. } => (), // TODO: Log?
                    DescriptionChanged { description } => state.description = description,
                    Completed => state.status = Status::Completed,
                    Archived => state.status = Status::Archived,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chronicle_commander::Aggregate;

    // TODO: These tests are very reptitive - would property based testing help?

    #[test]
    fn handles_initial_create() {
        let command = Command::Create("hi".to_string());

        assert_eq!(Task::handle_command(&None, command),
                   Ok(vec![Event::Created { description: "hi".to_string() }]));
    }

    #[test]
    fn handles_initial_change_description() {
        let command = Command::ChangeDescription("hi".to_string());

        assert_eq!(Task::handle_command(&None, command),
                   Err(CommandError::NotYetCreated));
    }

    #[test]
    fn handles_initial_completed() {
        assert_eq!(Task::handle_command(&None, Command::Complete),
                   Err(CommandError::NotYetCreated));
    }

    #[test]
    fn handles_initial_archive() {
        assert_eq!(Task::handle_command(&None, Command::Archive),
                   Err(CommandError::NotYetCreated));
    }

    #[test]
    fn handles_create() {
        let state = State::new("hi".to_string(), Status::Active);
        let command = Command::Create("yoho".to_string());

        assert_eq!(Task::handle_command(&Some(state), command),
                   Err(CommandError::AlreadyCreated));
    }

    #[test]
    fn handles_change_description_if_different() {
        let state = State::new("hi".to_string(), Status::Active);
        let command = Command::ChangeDescription("yoho".to_string());

        assert_eq!(Task::handle_command(&Some(state), command),
                   Ok(vec![Event::DescriptionChanged { description: "yoho".to_string() }]));
    }

    #[test]
    fn ignores_change_description_on_no_change() {
        let state = State::new("hi".to_string(), Status::Active);
        let command = Command::ChangeDescription("hi".to_string());

        assert_eq!(Task::handle_command(&Some(state), command), Ok(vec![]));
    }

    #[test]
    fn handles_complete_if_different() {
        let state = State::new("hi".to_string(), Status::Active);

        assert_eq!(Task::handle_command(&Some(state), Command::Complete),
                   Ok(vec![Event::Completed]));
    }

    #[test]
    fn ignores_complete_on_no_change() {
        let state = State::new("hi".to_string(), Status::Completed);

        assert_eq!(Task::handle_command(&Some(state), Command::Complete),
                   Ok(vec![]));
    }

    #[test]
    fn handles_archive_if_different() {
        let state = State::new("hi".to_string(), Status::Active);

        assert_eq!(Task::handle_command(&Some(state), Command::Archive),
                   Ok(vec![Event::Archived]));
    }

    #[test]
    fn ignores_archive_on_no_change() {
        let state = State::new("hi".to_string(), Status::Archived);

        assert_eq!(Task::handle_command(&Some(state), Command::Archive),
                   Ok(vec![]));
    }

    #[test]
    fn applies_initial_created() {
        let mut state = None;
        let event = Event::Created { description: "hi".to_string() };
        Task::apply_event(&mut state, event);

        assert_eq!(state, Some(State::new("hi".to_string(), Status::Active)));
    }

    #[test]
    fn applies_created() {
        let mut state = Some(State::new("hi".to_string(), Status::Active));
        let event = Event::Created { description: "HELLO".to_string() };
        Task::apply_event(&mut state, event);

        assert_eq!(state, Some(State::new("hi".to_string(), Status::Active)));
    }

    // TODO: more tests?
}

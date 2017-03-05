use chronicle_domain::Aggregate;

use super::Money;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    MoneyTransferRequested {
        debit_account: String,
        credit_account: String,
        amount: Money,
    },
}

#[derive(Debug, Clone)]
pub enum Command {
    TransferMoney {
        debit_account: String,
        credit_account: String,
        amount: Money,
    },
}

#[derive(Debug, Clone)]
pub enum CommandError {}

#[derive(Debug, Clone)]
pub enum Status {
    Requested,
}

#[derive(Debug, Clone)]
pub struct State {
    pub transfer_uuid: String,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: Money,
    pub state: Status,
}

pub struct Transfer;

impl Aggregate for Transfer {
    type State = Option<State>;
    type Event = Event;
    type Command = Command;
    type CommandError = CommandError;
    type EventsFuture = Result<Vec<Event>, CommandError>;

    fn initial_state() -> Option<State> {
        None
    }

    fn handle_command(_state: &Option<State>,
                      _command: Command)
                      -> Result<Vec<Event>, CommandError> {
        unimplemented!()
    }

    fn apply_event(_state: &mut Option<State>, _command: Event) {
        unimplemented!()
    }
}

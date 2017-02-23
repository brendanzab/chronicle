use chronicle_commander::Aggregate;

use super::Money;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Event {
    MoneyTransferRequested {
        debit_account: String,
        credit_account: String,
        amount: Money,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Command {
    TransferMoney {
        debit_account: String,
        credit_account: String,
        amount: Money,
    },
}

#[derive(Debug, Clone)]
pub enum ValidationError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TransferState {
    Requested,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub transfer_uuid: String,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: Money,
    pub state: TransferState,
}

pub struct TransferAggregate;

impl Aggregate for TransferAggregate {
    type State = Option<State>;
    type Event = Event;
    type Command = Command;
    type ValidationError = ValidationError;

    fn initial_state() -> Option<State> {
        None
    }

    fn apply_event(_state: &mut Option<State>, _command: Event) {
        unimplemented!()
    }

    fn handle_command(_state: &Option<State>,
                      _command: Command)
                      -> Result<Vec<Event>, ValidationError> {
        unimplemented!()
    }
}

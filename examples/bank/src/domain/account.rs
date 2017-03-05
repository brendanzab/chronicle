use chronicle_commander::Aggregate;
use futures::future::FutureResult;

use super::Money;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Event {
    AccountOpened { initial_balance: Money },
    MoneyDeposited {
        transfer_id: String,
        amount: Money,
        balance: Money,
    },
    MoneyWithdrawn {
        transfer_id: String,
        amount: Money,
        balance: Money,
    },
    AccountOverdrawn { balance: Money },
    AccountClosed,
}

#[derive(Debug, Clone)]
pub enum Command {
    OpenAccount { initial_balance: Money },
    DepositMoney { transfer_id: String, amount: Money },
    WithdrawMoney { transfer_id: String, amount: Money },
    CloseAccount,
}

#[derive(Debug, Clone)]
pub enum ValidationError {}

#[derive(Debug, Clone)]
pub enum Status {
    Active,
    Closed,
}

#[derive(Debug, Clone)]
pub struct State {
    pub balance: Money,
    pub status: Status,
}

pub struct Account;

impl Aggregate for Account {
    type State = Option<State>;
    type Event = Event;
    type Command = Command;
    type ValidationError = ValidationError;
    type EventsFuture = FutureResult<Vec<Event>, ValidationError>;

    fn initial_state() -> Option<State> {
        None
    }

    fn handle_command(_state: &Option<State>,
                      _command: Command)
                      -> FutureResult<Vec<Event>, ValidationError> {
        unimplemented!()
    }

    fn apply_event(_state: &mut Option<State>, _command: Event) {
        unimplemented!()
    }
}

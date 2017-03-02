use chronicle_commander::Aggregate;

use super::Money;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
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
pub enum CommandError {}

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

use chronicle_commander::Aggregate;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Command {
    OpenAccount { initial_balance: Money },
    DepositMoney { transfer_id: String, amount: Money },
    WithdrawMoney { transfer_id: String, amount: Money },
    CloseAccount,
}

#[derive(Debug, Clone)]
pub enum ValidationError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccountState {
    Active,
    Closed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub balance: Money,
    pub state: AccountState,
}

pub struct Account;

impl Aggregate for Account {
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

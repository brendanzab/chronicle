use futures::{Async, Future, IntoFuture, Poll, Sink, Stream};
use futures::sync::{mpsc, oneshot};
use uuid::Uuid;

use Aggregate;


/// A message to be placed in the queue on the command processor. It stores a
/// future to be resolved once the command has been processed.
struct CommandMessage<A>
    where A: Aggregate
{
    /// The unique idetifier of the aggregate to be operated on
    aggregate_id: Uuid,
    /// The command to be processed
    command: A::Command,
    /// A sender to allow the new state of the command to be yielded to the task
    /// that sent the command.
    tx: Option<oneshot::Sender<Result<A::State, A::CommandError>>>,
}


#[derive(Debug, Clone)]
pub enum CommandSenderError {
    OneshotCancelled,
    CommandProcessorCancelled,
}


/// A handle to which commands can be sent to a `CommandProcessor`.
pub struct CommandSender<A>
    where A: Aggregate
{
    message_tx: mpsc::Sender<CommandMessage<A>>,
}


impl<A> CommandSender<A>
    where A: Aggregate
{
    /// Send a command to be processed by a `CommandProcessor`, returning a
    /// future that yeilds the new state of the aggregate.
    pub fn execute_command
        (self,
         aggregate_id: Uuid,
         command: A::Command)
         -> impl Future<Item = Result<A::State, A::CommandError>, Error = CommandSenderError> {
        use self::CommandSenderError::*;

        let (tx, rx) = oneshot::channel();

        let message = CommandMessage {
            aggregate_id: aggregate_id,
            command: command,
            tx: Some(tx),
        };

        self.message_tx
            .send(message)
            .map_err(|_| CommandProcessorCancelled)
            .and_then(|_| rx.map_err(|_| OneshotCancelled))
    }
}


pub struct CommandProcessor<A>
    where A: Aggregate
{
    message_rx: mpsc::Receiver<CommandMessage<A>>,
}


impl<A> CommandProcessor<A>
    where A: Aggregate
{
    pub fn channel() -> (CommandSender<A>, CommandProcessor<A>) {
        let (tx, rx) = mpsc::channel(100); // FIXIME: Backpressure??
        (CommandSender { message_tx: tx }, CommandProcessor { message_rx: rx })
    }

    fn process_message(&self, message: CommandMessage<A>) -> Result<A::State, A::CommandError> {
        let state = unimplemented!();
        A::handle_command(&state, message.command)
    }
}


impl<A> Stream for CommandProcessor<A>
    where A: Aggregate
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Option<()>, ()> {
        self.message_rx.poll().map(|poll| match poll {
                                       Async::Ready(message) => Async::Ready(unimplemented!()),
                                       Async::NotReady => Async::NotReady,
                                   })
    }
}

// loop {
//     select! {
//         msg <- commands_a => unimplemented!(),
//         msg <- commands_b => unimplemented!(),
//         msg <- commands_c => unimplemented!(),
//         msg <- commands_d => unimplemented!(),
//     }
// }

// trait CommandBus {
//     // add code here
// }

// trait CommandSink {
//     type CommandFuture: Future;
//     pub fn call(&self, aggregate_id: Uuid, command: A::Command) -> CommandFuture;
// }

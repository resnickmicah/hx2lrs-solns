use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, SendError, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: crate::data::TicketDraft,
        response_sender: Sender<crate::store::TicketId>,
    },
    Get {
        id: crate::store::TicketId,
        response_sender: Sender<Option<crate::data::Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                if let Some(ticket) = store.get(id) {
                    let _ = response_sender.send(Some(ticket.to_owned()));
                };
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

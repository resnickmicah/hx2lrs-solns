use serde::{Deserialize, Serialize};
use sqlx::FromRow;

mod description;
mod status;
mod title;

pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct Ticket {
    pub id: i32,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

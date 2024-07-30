
mod description;
mod status;
mod title;

pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone, Serialize, FromRow)]
pub struct Ticket {
    pub id: i32,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

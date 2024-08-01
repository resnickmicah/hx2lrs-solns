use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

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

#[derive(Debug, PartialEq, Clone, Serialize, FromRow)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

pub struct TicketState {
    pool: PgPool,
}

#[post("/", data = "<data>")]
pub async fn create(
    data: Json<TicketDraft>,
    state: &State<TicketState>,
) -> Result<Json<Ticket>, BadRequest<String>> {
    let status = Status::ToDo;
    let description = TicketDescription::try_from(data.description.clone())
        .map_err(|e| BadRequest(e.to_string()))?;
    let title = TicketTitle::try_from(data.title.clone()).map_err(|e| BadRequest(e.to_string()))?;
    let ticket = Ticket {
        title,
        description,
        status,
    };
    let ticket_row = sqlx::query!(
            "INSERT INTO tickets(description,title,status) VALUES ($1,$2,$3) RETURNING description, title, status",
            String::from(ticket.description),
            String::from(ticket.title),
            String::from(ticket.status)
        )
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(e.to_string()))?;
    // Can trust that these are valid values because we just inserted them after validation above
    let ticket = Ticket {
        title: TicketTitle::try_from(ticket_row.title).unwrap(),
        description: TicketDescription::try_from(ticket_row.description).unwrap(),
        status: Status::try_from(ticket_row.status).unwrap(),
    };
    Ok(Json(ticket))
}

#[get("/<id>")]
pub async fn read(id: i32, state: &State<TicketState>) -> Result<Json<Ticket>, BadRequest<String>> {
    let ticket_row = sqlx::query!(
        "SELECT description, title, status FROM tickets WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| BadRequest(e.to_string()))?;
    let ticket = Ticket {
        title: TicketTitle::try_from(ticket_row.title).unwrap(),
        description: TicketDescription::try_from(ticket_row.description).unwrap(),
        status: Status::try_from(ticket_row.status).unwrap(),
    };

    Ok(Json(ticket))
}

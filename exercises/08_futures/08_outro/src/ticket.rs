use rocket::http::Status;
use rocket::response;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

mod description;
mod status;
mod title;

pub use description::TicketDescription;
pub use status::Status as TicketStatus;
pub use title::TicketTitle;

#[derive(Deserialize)]
pub struct TicketDraft {
    title: String,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, FromRow)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: TicketStatus,
}

pub struct TicketState {
    pub pool: PgPool,
}

#[derive(Debug)]
pub enum TicketHandlerError {
    NotFound(i32),
    ValidationFailure(String),
    DatabaseError(String),
    InvalidData(String),
}

impl<'r> Responder<'r, 'static> for TicketHandlerError {
    fn respond_to(self, r: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            TicketHandlerError::NotFound(id) => response::status::Custom(
                Status::NotFound,
                format!("Ticket with ID {} not found", id),
            )
            .respond_to(r),
            TicketHandlerError::ValidationFailure(e) => {
                response::status::Custom(Status::BadRequest, e).respond_to(r)
            }
            TicketHandlerError::DatabaseError(e) | TicketHandlerError::InvalidData(e) => {
                response::status::Custom(Status::InternalServerError, e).respond_to(r)
            }
        }
    }
}

#[post("/", data = "<data>")]
pub async fn create(
    data: Json<TicketDraft>,
    state: &State<TicketState>,
) -> Result<Json<Ticket>, TicketHandlerError> {
    let status = TicketStatus::ToDo;
    let description = TicketDescription::try_from(data.description.clone())
        .map_err(|e| TicketHandlerError::ValidationFailure(e.to_string()))?;
    let title = TicketTitle::try_from(data.title.clone())
        .map_err(|e| TicketHandlerError::ValidationFailure(e.to_string()))?;
    let ticket_row = sqlx::query!(
            "INSERT INTO tickets(description,title,status) VALUES ($1,$2,$3) RETURNING description, title, status",
            String::from(description),
            String::from(title),
            String::from(status)
        )
        .fetch_one(&state.pool)
        .await
        .map_err(|e| TicketHandlerError::DatabaseError(e.to_string()))?;
    let ticket = Ticket {
        title: TicketTitle::try_from(ticket_row.title)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
        description: TicketDescription::try_from(ticket_row.description)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
        status: TicketStatus::try_from(ticket_row.status)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
    };
    Ok(Json(ticket))
}

#[get("/<id>")]
pub async fn read(id: i32, state: &State<TicketState>) -> Result<Json<Ticket>, TicketHandlerError> {
    let get_ticket_query_result = sqlx::query!(
        "SELECT description, title, status FROM tickets WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await;

    let ticket_row = match get_ticket_query_result {
        Ok(ticket_row) => ticket_row,
        Err(sqlx::Error::RowNotFound) => return Err(TicketHandlerError::NotFound(id)),
        Err(e) => return Err(TicketHandlerError::InvalidData(e.to_string())),
    };

    let ticket = Ticket {
        title: TicketTitle::try_from(ticket_row.title)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
        description: TicketDescription::try_from(ticket_row.description)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
        status: TicketStatus::try_from(ticket_row.status)
            .map_err(|e| TicketHandlerError::InvalidData(e.to_string()))?,
    };

    Ok(Json(ticket))
}

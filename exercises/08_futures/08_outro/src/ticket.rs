use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, PgPool, Row};

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

#[derive(Debug, PartialEq, Clone, Serialize, sqlx::Type)]
pub struct Ticket {
    pub id: i32,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

pub struct TicketState {
    pool: PgPool,
}

// #[post("/", data = "<data>")]
// pub async fn create(
//     data: Json<TicketDraft>,
//     state: &State<TicketState>,
// ) -> Result<Json<i32>, BadRequest<String>> {
//     let status = Status::ToDo;
//     let id: i32 = sqlx::query_as!(
//             Ticket,
//             "INSERT INTO tickets(description,title,status) VALUES ($1,$2,$3) RETURNING id",
//             &data.description.0,
//             &data.title.0,
//             &status
//         )
//         .fetch_one(&state.pool)
//         .await
//         .map_err(|e| BadRequest(e.to_string()))?;

//     Ok(Json(id))
// }

// #[get("/<id>")]
// pub async fn read(id: i32, state: &State<TicketState>) -> Result<Json<Ticket>, BadRequest<String>> {
//     let ticket = sqlx::query_as!(Ticket, "SELECT * FROM tickets WHERE id = $1", id)
//         .fetch_one(&state.pool)
//         .await
//         .map_err(|e| BadRequest(e.to_string()))?;

//     Ok(Json(ticket))
// }

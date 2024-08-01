#[macro_use]
extern crate rocket;

mod ticket;

use sqlx::PgPool;
use ticket::{create, read, TicketState};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "{secrets.DATABASE_URL}")] pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    let state = TicketState { pool };
    let rocket = rocket::build()
        .mount("/ticket", routes![create, read])
        .manage(state);

    Ok(rocket.into())
}

#[macro_use]
extern crate rocket;

mod ticket;

use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use ticket::{create, read, TicketState};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://{secrets.POSTGRES_USER}:{secrets.POSTGRES_PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../tickets_schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = TicketState { pool };
    let rocket = rocket::build()
        .mount("/ticket", routes![create, read])
        .manage(state);

    Ok(rocket.into())
}

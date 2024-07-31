#[macro_use]
extern crate rocket;

mod ticket;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{Executor, FromRow, PgPool};
// use ticket::{create, read};

struct MyState {
    pool: PgPool,
}

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

    let state = MyState { pool };
    let rocket = rocket::build().mount("/ticket", routes![]).manage(state);

    Ok(rocket.into())
}

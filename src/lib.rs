use rocket_db_pools::{sqlx, Database};
use rocket::serde::Deserialize;

// pub mod admin;
pub mod public;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub username: String,
    pub password: String,
}

#[derive(Database)]
#[database("thymesheet")]
pub struct Thymesheet(sqlx::SqlitePool);

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Week {
    id: i32,
    body: String,
}

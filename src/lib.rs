use diesel::{Connection, SqliteConnection};
use rocket::serde::Deserialize;

pub mod models;
pub mod schema;

pub mod admin;
pub mod public;

use rocket_sync_db_pools::{database, diesel};

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&database_url).unwrap()
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub username: String,
    pub password: String,
}

#[database("thymesheet")]
pub struct Thymesheet(diesel::SqliteConnection);

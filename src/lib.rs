use diesel::{Connection, SqliteConnection};

pub mod models;
pub mod schema;

pub mod admin;
pub mod public;

pub fn establish_connection() -> SqliteConnection {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&database_url).unwrap()
}

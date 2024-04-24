use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request;
use rocket::serde::Deserialize;
use rocket::{request::FromRequest, request::Request};
use rocket_db_pools::{sqlx, Database};

// pub mod admin;
pub mod admin;
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

#[derive(rocket::form::FromForm)]
pub struct WeekForm<'a> {
    pub id: i32,
    pub body: &'a str,
}

#[derive(Debug)]
pub enum AppError {
    WhoAreYou,
    InternalError,
}

pub struct AdminUser {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("admin")
            .and_then(|cookie| cookie.value().parse::<usize>().ok())
            .and_then(|_| Some(AdminUser {}))
            .or_error((Status::Forbidden, AppError::WhoAreYou))
    }
}

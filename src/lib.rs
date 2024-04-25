use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request;
use rocket::response::status::NotFound;
use rocket::serde::Deserialize;
use rocket::{request::FromRequest, request::Request};
use rocket_db_pools::Connection;
use rocket_db_pools::{sqlx, Database};
use rocket_dyn_templates::{context, Template};

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

pub async fn get_weeks(mut db: Connection<Thymesheet>) -> Vec<Week> {
    let mut results: Vec<Week> = Vec::new();
    let res: Result<Vec<Week>, _> = sqlx::query_as("SELECT id, body FROM weeks")
        .fetch_all(&mut **db)
        .await;

    match res {
        Err(_) => { /* nothing happens. */ }
        Ok(mut w) => results.append(&mut w),
    }

    results
}

pub async fn render_week(
    week: i32,
    template: String,
    mut db: Connection<Thymesheet>,
) -> Result<Template, NotFound<String>> {
    let mut results: Vec<Week> = Vec::new();
    let res = sqlx::query_as("SELECT id, body FROM weeks WHERE id = ?")
        .bind(week)
        .fetch_one(&mut **db)
        .await;

    match res {
        Err(_) => {
            return Err(NotFound("Week Not Found".to_string()));
        }
        Ok(w) => {
            results.push(w);

            Ok(Template::render(
                template,
                context! {weeks: &results, admin: false, title: format!("Week {}", week)},
            ))
        }
    }
}

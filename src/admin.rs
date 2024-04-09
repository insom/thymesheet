#[allow(unused)]
use diesel::prelude::*;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request;
use rocket::{get, request::FromRequest, request::Request};
use rocket_dyn_templates::{context, Template};

pub struct AdminUser {}

#[derive(Debug)]
pub enum AppError {
    WhoAreYou,
    InternalError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get("admin")
            .and_then(|cookie| cookie.value().parse::<usize>().ok())
            .and_then(|_| Some(AdminUser {}))
            .or_error((Status::Forbidden, AppError::WhoAreYou))
    }
}

#[get("/admin")]
pub fn index(_admin: AdminUser) -> Template {
    let v = Vec::<bool>::new();
    Template::render("index", context! {weeks: &v, admin: true})
}

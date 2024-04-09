use std::fmt::Display;
use std::fmt;

#[allow(unused)]
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, request::FromRequest, request::Request};
use rocket::request;
use rocket_dyn_templates::{context, Template};

pub struct AdminUser {}

#[derive(Debug)]
pub enum AppError {
    MissingQuery,
    MissingFilename,
    ConfigLoad,
}
impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(POTATO)")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AppError;

    async fn from_request(_req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Error((Status::NotFound, AppError::MissingQuery))
    }
}

#[get("/admin")]
pub fn index(_admin: AdminUser) -> Template {
    let v = Vec::<bool>::new(); // an empty vec needs to have type information so it can be serailized #smh
    Template::render("index", context! {weeks: &v})
}

#[allow(unused)]
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::http::{CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request;
use rocket::response::Redirect;
use rocket::{get, post, request::FromRequest, request::Request};
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
            .get_private("admin")
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

#[get("/admin/login")]
pub fn login_get() -> Template {
    Template::render("login", context! {})
}

#[derive(FromForm)]
pub struct LoginForm<'r> {
    username: &'r str,
    password: &'r str,
}

#[post("/admin/login", data = "<login>")]
pub fn login(login: Form<LoginForm<'_>>, cookies: &CookieJar<'_>) -> Redirect {
    if login.username == "test" && login.password == "test" {
        cookies.add_private(("admin", "1"));
        return Redirect::to("/admin");
    }
    Redirect::to("/admin/login")
}

#[get("/admin/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("admin");
    return Redirect::to("/admin/login");
}

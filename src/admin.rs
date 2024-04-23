use crate::models::{Week, WeekForm};
use crate::schema::weeks::dsl::{body, id, weeks};
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::http::{CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::uri;
use rocket::State;
use rocket::{catch, get, post, request::FromRequest, request::Request};
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

#[get("/")]
pub fn index(_admin: AdminUser) -> Template {
    let mut connection = crate::establish_connection();
    let results = weeks
        .select(Week::as_select())
        .load(&mut connection)
        .unwrap();
    Template::render("admin/index", context! {weeks: &results, admin: true})
}

#[post("/week/<week>", data = "<week_form>")]
pub fn week(
    week: i32,
    week_form: Form<WeekForm<'_>>,
    _admin: AdminUser,
) -> Result<Redirect, NotFound<String>> {
    let mut connection = crate::establish_connection();
    let result: Result<Week, _> = weeks
        .find(week)
        .select(Week::as_select())
        .first(&mut connection);

    match result {
        Ok(_) => {
            diesel::update(weeks.find(week))
                .set(body.eq(week_form.body))
                .execute(&mut connection)
                .unwrap();
            Ok(Redirect::to(format!("/admin/week/{}", week_form.id)))
        }
        Err(_) => {
            Err(NotFound("Week Not Found".to_string()))
        }
    }
}

#[get("/week/<week>")]
pub fn week_get(week: i32, _admin: AdminUser) -> Result<Template, NotFound<String>> {
    let mut connection = crate::establish_connection();
    let results: Vec<_> = weeks
        .select(Week::as_select())
        .filter(id.eq(week))
        .load(&mut connection)
        .unwrap();

    if results.len() == 0 {
        return Err(NotFound("Week Not Found".to_string()));
    }
    Ok(Template::render(
        "admin/week",
        context! {weeks: &results, admin: true},
    ))
}

#[catch(403)]
pub fn redir_to_login() -> Redirect {
    return Redirect::to(uri!("/admin", login_get));
}

#[get("/login")]
pub fn login_get() -> Template {
    Template::render("login", context! {})
}

#[derive(FromForm)]
pub struct LoginForm<'r> {
    username: &'r str,
    password: &'r str,
}

#[post("/login", data = "<login>")]
pub fn login(
    login: Form<LoginForm<'_>>,
    cookies: &CookieJar<'_>,
    config: &State<crate::Config>,
) -> Redirect {
    if login.username == config.username && login.password == config.password {
        cookies.add_private(("admin", "1"));
        return Redirect::to("/admin");
    }
    return Redirect::to(uri!("/admin", login_get));
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("admin");
    return Redirect::to(uri!("/admin", login_get));
}

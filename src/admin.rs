use rocket::form::{Form, FromForm};
use rocket::http::{CookieJar};
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::uri;
use rocket::State;
use rocket::{catch, get, post};
use rocket_dyn_templates::{context, Template};
use crate::{AdminUser, Week, WeekForm, Thymesheet};
use rocket_db_pools::Connection;

#[get("/")]
pub async fn index(_admin: AdminUser, mut db: Connection<Thymesheet>) -> Template {
    let mut results: Vec<Week> = Vec::new();
    let res: Result<Vec<Week>, _> = sqlx::query_as("SELECT id, body FROM weeks")
        .fetch_all(&mut **db)
        .await;

    match res {
        Err(_) => { /* nothing happens. */ }
        Ok(mut w) => results.append(&mut w),
    }
    Template::render("admin/index", context! {weeks: &results, admin: false})
}

#[post("/week/<week>", data = "<week_form>")]
pub async fn week(
    week: i32,
    week_form: Form<WeekForm<'_>>,
    _admin: AdminUser, mut db: Connection<Thymesheet>
) -> Result<Redirect, NotFound<String>> {
    let result = sqlx::query("SELECT id, body FROM weeks WHERE id = ?")
        .bind(week)
        .fetch_one(&mut **db)
        .await;

    match result {
        Ok(_) => {
            // FIXME AWDB
            Ok(Redirect::to(format!("/admin/week/{}", week_form.id)))
        }
        Err(_) => {
            Err(NotFound("Week Not Found".to_string()))
        }
    }
}

#[get("/week/<week>")]
pub async fn week_get(week: i32, _admin: AdminUser, mut db: Connection<Thymesheet>
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
                "admin/week",
                context! {weeks: &results, admin: false, title: format!("Week {}", week)},
            ))
        }
    }
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

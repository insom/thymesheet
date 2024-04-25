use crate::{AdminUser, Thymesheet, WeekForm};
use rocket::form::{Form, FromForm};
use rocket::http::CookieJar;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::uri;
use rocket::State;
use rocket::{catch, get, post};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index(_admin: AdminUser, db: Connection<Thymesheet>) -> Template {
    let results = crate::get_weeks(db).await;
    Template::render("admin/index", context! {weeks: &results})
}

#[post("/week/<week>", data = "<week_form>")]
pub async fn week(
    week: i32,
    week_form: Form<WeekForm<'_>>,
    _admin: AdminUser,
    mut db: Connection<Thymesheet>,
) -> Result<Redirect, NotFound<String>> {
    let result = sqlx::query("SELECT id, body FROM weeks WHERE id = ?")
        .bind(week)
        .fetch_one(&mut **db)
        .await;

    match result {
        Ok(_) => {
            sqlx::query("UPDATE weeks SET body = ? WHERE id = ?")
                .bind(week_form.body)
                .bind(week)
                .execute(&mut **db)
                .await
                .unwrap();
            Ok(Redirect::to(format!("/admin/week/{}", week_form.id)))
        }
        Err(_) => Err(NotFound("Week Not Found".to_string())),
    }
}

#[post("/week", data = "<week_form>")]
pub async fn new_week(
    week_form: Form<WeekForm<'_>>,
    _admin: AdminUser,
    mut db: Connection<Thymesheet>,
) -> Result<Redirect, NotFound<String>> {
    sqlx::query("INSERT INTO weeks (id, body) VALUES (?, ?)")
        .bind(week_form.id)
        .bind(week_form.body)
        .execute(&mut **db)
        .await
        .unwrap();
    Ok(Redirect::to(format!("/admin/week/{}", week_form.id)))
}

#[get("/week")]
pub async fn new_week_get(_admin: AdminUser) -> Template {
    Template::render("admin/week", context! {weeks: [0]})
}

#[get("/week/<week>")]
pub async fn week_get(
    week: i32,
    _admin: AdminUser,
    db: Connection<Thymesheet>,
) -> Result<Template, NotFound<String>> {
    crate::render_week(week, String::from("admin/week"), db).await
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

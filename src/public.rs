use crate::{Thymesheet};
use sqlx::prelude::*;
use rocket::get;
use rocket::response::status::NotFound;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn index(admin: Option<AdminUser>) -> Template {
    let results: Vec<bool> = Vec::new();

    Template::render("index", context! {weeks: &results, admin: admin.is_some()})
}

#[get("/week/<week>")]
pub async fn week(week: i32, admin: Option<AdminUser>) -> Result<Template, NotFound<String>> {
    let results: Vec<bool> = Vec::new();

    if results.len() == 0 {
        return Err(NotFound("Week Not Found".to_string()));
    }

    Ok(Template::render(
        "index",
        context! {weeks: &results, admin: admin.is_some(), title: format!("Week {}", week)},
    ))
}

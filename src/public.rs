use crate::{Thymesheet};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{Row, query};
use rocket::get;
use rocket::response::status::NotFound;
use rocket_dyn_templates::{context, Template};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Week {
    id: i32,
    body: String,
}

#[get("/")]
pub async fn index(mut db: Connection<Thymesheet>) -> Template {
    let mut results: Vec<Week> = Vec::new();
    let w: Week = sqlx::query_as("SELECT id, body FROM weeks").fetch_one(&mut **db).await.unwrap();
    results.push(w);

    Template::render("index", context! {weeks: &results, admin: false})
}

#[get("/week/<week>")]
pub async fn week(week: i32) -> Result<Template, NotFound<String>> {
    let results: Vec<bool> = Vec::new();

    if results.len() == 0 {
        return Err(NotFound("Week Not Found".to_string()));
    }

    Ok(Template::render(
        "index",
        context! {weeks: &results, admin: false, title: format!("Week {}", week)},
    ))
}

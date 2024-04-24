use crate::Thymesheet;
use rocket::get;
use rocket::response::status::NotFound;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Week {
    id: i32,
    body: String,
}

#[get("/")]
pub async fn index(mut db: Connection<Thymesheet>) -> Template {
    let mut results: Vec<Week> = Vec::new();
    let res: Result<Vec<Week>, _> = sqlx::query_as("SELECT id, body FROM weeks")
        .fetch_all(&mut **db)
        .await;

    match res {
        Err(_) => { /* nothing happens. */ }
        Ok(mut w) => {
            results.append(&mut w)
        }
    }
    Template::render("index", context! {weeks: &results, admin: false})
}

#[get("/week/<week>")]
pub async fn week(week: i32, mut db: Connection<Thymesheet>) -> Result<Template, NotFound<String>> {
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
                "index",
                context! {weeks: &results, admin: false, title: format!("Week {}", week)},
            ))
        }
    }
}
